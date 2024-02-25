use std::{collections::HashMap, fmt};

use nu_ansi_term::Color;
use tracing::{
    field::{Field, Visit},
    Event, Subscriber,
};
use tracing_subscriber::{
    fmt::{format::Writer, FmtContext, FormatEvent, FormatFields},
    registry::LookupSpan,
};

pub struct Format;

struct Visitor<'a> {
    out: &'a mut String,
    db_statement: String,
    last_sql: bool,
}

impl<'a> Visitor<'a> {
    fn new(out: &'a mut String) -> Self {
        Self {
            out,
            db_statement: String::new(),
            last_sql: false,
        }
    }
}

impl<'a> Visit for Visitor<'a> {
    fn record_debug(&mut self, field: &Field, value: &dyn fmt::Debug) {
        let italic = nu_ansi_term::Style::new().italic().reset_before_style();
        let dimmed = nu_ansi_term::Style::new().dimmed().reset_before_style();

        let name = field.name();
        match name {
            "rows_affected" | "rows_returned" | "elapsed" => {
                self.out
                    .push_str(&italic.paint(format!("{}", name)).to_string());
                self.out.push_str(&dimmed.paint("=").to_string());
                self.out.push_str(&format!("{:?}\n", value));
            }
            "db.statement" => {
                self.db_statement = format!("{:?}", value)
                    .replace("\\n", "\n")
                    .replace("\\\"", "")
                    .trim_start()
                    .trim_end_matches("\"")
                    .to_string();
            }
            _ => {}
        }
    }
}

fn format_event_sqlx<S, N>(
    _: &FmtContext<'_, S, N>,
    mut writer: Writer<'_>,
    event: &Event<'_>,
) -> std::fmt::Result
where
    S: Subscriber + for<'a> LookupSpan<'a>,
    N: for<'a> FormatFields<'a> + 'static,
{
    let mut out = String::new();
    let mut visitor = Visitor::new(&mut out);
    event.record(&mut visitor);

    #[derive(Clone, Copy, PartialEq, Debug)]
    enum CharType {
        Other,
        Numeral,
        Identifier,
        Uppercase,
        Newline,
    }
    use CharType::*;

    let style = |ct| {
        let s = nu_ansi_term::Style::new();
        match ct {
            Other => s.dimmed(),
            Numeral => s.fg(Color::LightGreen),
            Identifier => s.bold(),
            Uppercase => s.bold().fg(Color::Purple),
            Newline => s,
        }
        .reset_before_style()
    };
    let Visitor {
        out, db_statement, ..
    } = visitor;

    db_statement
        .chars()
        .map(|c| {
            let ty = match c {
                '\n' => Newline,
                '0'..='9' | '$' => Numeral,
                'a'..='z' | '.' | '_' => Identifier,
                'A'..='Z' => Uppercase,
                _ => Other,
            };
            (c, ty)
        })
        .map_windows(|[(_, ta), (b, tb)]| {
            if *tb == Newline {
                return Ok(());
            }
            if *ta == Newline && *tb == Uppercase {
                writeln!(writer)?;
            }
            if ta != tb {
                writer.write_str(
                    &style(*tb)
                        .paint(format!("{}", b))
                        .to_string()
                        .as_str()
                        .trim_end_matches("\x1b[0m"),
                )
            } else {
                writer.write_char(*b)
            }
        })
        .collect::<std::fmt::Result>()?;
    writeln!(writer)?;
    writer.write_str(out)?;
    Ok(())
}

impl<S, N> FormatEvent<S, N> for Format
where
    S: Subscriber + for<'a> LookupSpan<'a>,
    N: for<'a> FormatFields<'a> + 'static,
{
    fn format_event(
        &self,
        ctx: &FmtContext<'_, S, N>,
        writer: Writer<'_>,
        event: &Event<'_>,
    ) -> std::fmt::Result {
        match event.metadata().module_path() {
            Some(module) if module.starts_with("sqlx") => format_event_sqlx(ctx, writer, event),
            _ => {
                tracing_subscriber::fmt::format::Format::default().format_event(ctx, writer, event)
            }
        }
    }
}

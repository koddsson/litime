use textwrap::{fill, Options};

use crate::minute::Minute;

static INITIAL_INDENT: &str = "  \" ";
static SUBSEQUENT_INDENT: &str = "    ";
static FOOTER_INDENT: &str = "        ";

fn get_colour(name: &str) -> &str {
    match name {
        "black" => "\u{1b}[30m",
        "red" => "\u{1b}[31m",
        "green" => "\u{1b}[32m",
        "yellow" => "\u{1b}[33m",
        "blue" => "\u{1b}[34m",
        "magenta" => "\u{1b}[35m",
        "cyan" => "\u{1b}[36m",
        "white" => "\u{1b}[37m",
        "bright-black" => "\u{1b}[90m",
        "bright-red" => "\u{1b}[91m",
        "bright-green" => "\u{1b}[92m",
        "bright-yellow" => "\u{1b}[93m",
        "bright-blue" => "\u{1b}[94m",
        "bright-magenta" => "\u{1b}[95m",
        "bright-cyan" => "\u{1b}[96m",
        "bright-white" => "\u{1b}[97m",
        _ => "\u{1b}[0m",
    }
}

impl Minute<'_> {
    pub fn formatted(&self, width: usize, main: &str, time: &str, author: &str) -> String {
        let quote = format!("\x02{}\x03{}\x02{}\x00", self.start, self.time, self.end);
        let footer = format!("\x01{} – {}\x00", self.author, self.title);

        let quote_options = Options::new(width)
            .initial_indent(INITIAL_INDENT)
            .subsequent_indent(SUBSEQUENT_INDENT);
        let footer_options = Options::new(width)
            .initial_indent(FOOTER_INDENT)
            .subsequent_indent(FOOTER_INDENT);

        let quote = fill(quote.as_str(), &quote_options);
        let footer = fill(footer.as_str(), &footer_options);

        format!("\n{}\n\n{}\n", quote, footer)
            .replace('\x00', get_colour("reset"))
            .replace('\x01', get_colour(author))
            .replace('\x02', get_colour(main))
            .replace('\x03', get_colour(time))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn wrapped_quote() {
        let minute = Minute {
            start: "black black black ",
            time: "red red red red",
            end: " black black black",
            author: "author",
            title: "title",
        };

        let formatted = minute.formatted(20, "bright-black", "red", "white");
        let expected = [
            format!("\n  \" {}black black", get_colour("bright-black")),
            format!("    black {}red red", get_colour("red")),
            format!("    red red{} black", get_colour("bright-black")),
            format!("    black black{}\n", get_colour("reset")),
            format!("        {}author –", get_colour("white")),
            format!("        title{}\n", get_colour("reset")),
        ]
        .join("\n");

        assert_eq!(formatted, expected);
    }

    #[test]
    fn short_quote() {
        let minute = Minute {
            start: "foo ",
            time: "bar",
            end: " baz",
            author: "author",
            title: "title",
        };

        let formatted = minute.formatted(50, "bright-black", "red", "white");
        let expected = [
            format!(
                "\n  \" {}foo {}bar{} baz{}\n",
                get_colour("bright-black"),
                get_colour("red"),
                get_colour("bright-black"),
                get_colour("reset")
            ),
            format!(
                "        {}author – title{}\n",
                get_colour("white"),
                get_colour("reset")
            ),
        ]
        .join("\n");

        assert_eq!(formatted, expected);
    }

    #[test]
    fn no_start() {
        let minute = Minute {
            start: "",
            time: "bar",
            end: " baz",
            author: "author",
            title: "title",
        };

        let formatted = minute.formatted(50, "bright-black", "red", "white");
        let expected = [
            format!(
                "\n  \" {}{}bar{} baz{}\n",
                get_colour("bright-black"),
                get_colour("red"),
                get_colour("bright-black"),
                get_colour("reset")
            ),
            format!(
                "        {}author – title{}\n",
                get_colour("white"),
                get_colour("reset")
            ),
        ]
        .join("\n");

        assert_eq!(formatted, expected);
    }

    #[test]
    fn no_end() {
        let minute = Minute {
            start: "foo ",
            time: "bar",
            end: "",
            author: "author",
            title: "title",
        };

        let formatted = minute.formatted(50, "bright-black", "red", "white");
        let expected = [
            format!(
                "\n  \" {}foo {}bar{}{}\n",
                get_colour("bright-black"),
                get_colour("red"),
                get_colour("bright-black"),
                get_colour("reset")
            ),
            format!(
                "        {}author – title{}\n",
                get_colour("white"),
                get_colour("reset")
            ),
        ]
        .join("\n");

        assert_eq!(formatted, expected);
    }

    #[test]
    fn issue_4() {
        let minute = Minute {
            start: "At 10.15 Arlena departed from her rondezvous, a minute or two later Patrick Redfern came down and registered surprise, annoyance, etc. Christine's task was easy enough. Keeping her own watch concealed she asked Linda at twenty-five past eleven what time it was. Linda looked at her watch and replied that it was a ",
            time: "quarter to twelve",
            end: ".",
            author: "Agatha Christie",
            title: "Evil under the Sun",
        };

        let formatted = minute.formatted(50, "bright-black", "red", "white");
        let expected = [
            format!(
                "\n  \" {}At 10.15 Arlena departed from her rondezvous,",
                get_colour("bright-black")
            ),
            String::from("    a minute or two later Patrick Redfern came"),
            String::from("    down and registered surprise, annoyance, etc."),
            String::from("    Christine\'s task was easy enough. Keeping her"),
            String::from("    own watch concealed she asked Linda at twenty-"),
            String::from("    five past eleven what time it was. Linda"),
            String::from("    looked at her watch and replied that it was a"),
            format!(
                "    {}quarter to twelve{}.{}\n",
                get_colour("red"),
                get_colour("bright-black"),
                get_colour("reset")
            ),
            format!(
                "        {}Agatha Christie – Evil under the Sun{}\n",
                get_colour("white"),
                get_colour("reset")
            ),
        ]
        .join("\n");

        assert_eq!(formatted, expected);
    }

    #[test]
    fn issue_6() {
        let minute = Minute {
            start: "And the first stop had been at ",
            time: "1.16pm",
            end: " which was 17 minutes later.",
            author: "Mark Haddon",
            title: "The Curious Incident of the Dog in the Night-Time",
        };

        let formatted = minute.formatted(30, "bright-black", "red", "white");
        let expected = [
            format!(
                "\n  \" {}And the first stop had",
                get_colour("bright-black")
            ),
            format!(
                "    been at {}1.16pm{} which was",
                get_colour("red"),
                get_colour("bright-black")
            ),
            format!("    17 minutes later.{}\n", get_colour("reset")),
            format!("        {}Mark Haddon – The", get_colour("white")),
            String::from("        Curious Incident of"),
            String::from("        the Dog in the Night-"),
            format!("        Time{}\n", get_colour("reset")),
        ]
        .join("\n");

        assert_eq!(formatted, expected);
    }
}

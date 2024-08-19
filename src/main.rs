use std::fmt::Display;

#[derive(Debug)]
struct AppSplashMessage {
    pub title: String,
    pub border_angles: String,
    pub border_x: String,
    pub border_y: String,
    pub padding_x: u16,
    pub padding_y: u16,
}

impl Default for AppSplashMessage {
    fn default() -> Self {
        AppSplashMessage {
            title: format!("App Title"),
            border_angles: format!("+"),
            border_x: format!("-"),
            border_y: format!("|"),
            padding_x: 2,
            padding_y: 0,
        }
    }
}

impl AppSplashMessage {
    pub fn from_title(title: &str) -> Self {
        Self {
            title: title.into(),
            ..Self::default()
        }
    }

    pub fn with_padding_y(title: &str, padding_y: u16) -> Self {
        Self {
            title: title.into(),
            padding_y,
            ..Self::default()
        }
    }
}

impl Display for AppSplashMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // represents a top or bottom line initalized
        // adding the left angle character
        let mut border_line = format!("{}", self.border_angles);

        // represents a blank horizontal line initalized
        // adding the left border character
        let mut blank_line = format!("{}", self.border_y);

        // quick padding text to be added multiple times
        let mut padding_x = String::new();

        for _ in 0..self.padding_x {
            padding_x += " ";

            // add 2 space and 2 border characters (1 for the start and
            // 1 for the end) to the border_line and and the blank_line
            // here calls are duplicated for clarity
            blank_line += " ";
            blank_line += " ";
            border_line += self.border_x.as_str();
            border_line += self.border_x.as_str()
        }

        // extend the blank line and the border line to cover the title
        // length
        for _ in self.title.chars() {
            blank_line += " ";
            border_line += self.border_x.as_str();
        }

        // add last characters for the right border of the blank line
        // and the border line
        blank_line += self.border_y.as_str();
        border_line += self.border_angles.as_str();

        // compose the textline cobining left and right borders+padding
        // with the title text
        let text_line = format!(
            "{}{}{}{}{}",
            self.border_y, padding_x, self.title, padding_x, self.border_y
        );

        // final printable line, to be composed adding vertical padding
        // and composing all together
        let mut print_line = format!("{}\n", border_line);

        // add top padding adding a blank line for each padding unit
        for _ in 0..self.padding_y {
            print_line += format!("{}\n", blank_line).as_str();
        }

        // add the title line
        print_line += format!("{}\n", text_line).as_str();

        // add bottom padding adding a blank line for each padding unit
        for _ in 0..self.padding_y {
            print_line += format!("{}\n", blank_line).as_str();
        }

        // add bottom border
        print_line += format!("{}\n", border_line).as_str();

        write!(f, "{}", print_line)
    }
}

#[tokio::main]
async fn main() {
    let message = AppSplashMessage::from_title("TANQUE WEBSITE - WELCOME");

    println!("{}", message.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn app_title() {
        let test_message = format!(
            "{}{}{}",
            "+----------------------------+\n",
            "|  TANQUE WEBSITE - WELCOME  |\n",
            "+----------------------------+\n",
        );
        let message = AppSplashMessage::from_title("TANQUE WEBSITE - WELCOME");

        assert_eq!(message.to_string(), test_message);
    }

    #[test]
    fn vertical_padding() {
        let test_message = format!(
            "{}{}{}{}{}",
            "+----------------------------+\n",
            "|                            |\n",
            "|  TANQUE WEBSITE - WELCOME  |\n",
            "|                            |\n",
            "+----------------------------+\n",
        );
        let message = AppSplashMessage::with_padding_y("TANQUE WEBSITE - WELCOME", 1);

        assert_eq!(message.to_string(), test_message);
    }
}

use std::borrow::Cow;

type CowStr = Cow<'static, str>;

struct Template<'a> {
    template: &'a str,

    title: CowStr,
    artist: CowStr,
    album: CowStr,
    comment: CowStr,
    genre: CowStr,
    year: CowStr,
    track_number: CowStr,
}

impl Default for Template<'static> {
    fn default() -> Self {
        let def = CowStr::default();
        Template {
            template: "%n - %t.mp3",
            title: def.clone(),
            artist: def.clone(),
            album: def.clone(),
            comment: def.clone(),
            genre: def.clone(),
            year: def.clone(),
            track_number: def,
        }
    }
}

impl<'a> ToString for Template<'a> {
    fn to_string(&self) -> String {
        self.template
            .replace("%t", &self.title)
            .replace("%a", &self.artist)
            .replace("%b", &self.album)
            .replace("%c", &self.comment)
            .replace("%g", &self.genre)
            .replace("%y", &self.year)
            .replace("%n", &self.track_number)
    }
}

macro_rules! replace_template {
    ( $( $x:ident ),* ) => (
        $(
            fn $x<S: Into<CowStr>>(self, $x: S) -> Self {
                Template {
                    $x: $x.into(),
                    ..self
                }
            }
        )*
    );
}

impl<'a> Template<'a> {
    fn with_template(template: &'a str) -> Template<'a> {
        Template {
            template,
            ..Default::default()
        }
    }

    replace_template!(title, artist, album, comment, genre, year, track_number);
}

#[cfg(test)]
mod tests {
    use super::Template;

    #[test]
    fn basic_functionality() {
        let t = Template::with_template("%n - %t.mp3");

        assert_eq!("01 - foo.mp3", t.title("foo")
                                    .artist("anon")
                                    .track_number("01")
                                    .to_string()
        );
    }
}

use iced::{Color, Element, color, theme::Mode, widget::{Column, Row, button, column, container, row, text}};

pub fn main() -> iced::Result {
    iced::run(Model::update, Model::view)
}

/// The struct contains all the data of the application. 
#[derive(Default)]
struct Model {
    pub bookmarks: Vec::<Bookmark>,
}

/// The struct models a bookmark. A name usefully to identify it, a description 
/// where to add some notes and the link, the real information of a bookmark.
#[derive(Debug, Clone)]
struct Bookmark {
    pub id: String,
    pub name: String,
    pub description: String,
    pub link: String,
}

#[derive(Debug, Clone)]
enum Message {
    Add(Bookmark),
    Remove(usize),
}


impl Model {

    fn update(&mut self, message: Message) {
        match message {
            Message::Add(bookmark) => {
                self.bookmarks.push(bookmark);
            }
            Message::Remove(index) => {
                self.bookmarks.remove(index);
            }
        }
    }

    //Column::new()
    //   .push(headers)
    //   .extend(content_rows)
    //   .push(new_row)
    //   .height(Length::Shrink)
    //   .into()

    fn view(&self) -> Column<'_,Message> {  
        //let mut items: Vec<Column<'_, Message>> = Vec::new();
        //for (index, bookmark) in self.bookmarks.iter().enumerate() {
        //    items.push(
        //        column![text(index), text(bookmark.name.clone()), text(bookmark.description.clone()), text(bookmark.link.as_str()), button("delete").on_press(Message::Remove(index))],
        //        
        //    );
        //}    
        let mut v = Vec::new();
        v.push(
            Bookmark{
                id: "x-g-9-e".to_string(),
                name: "google".to_string(), 
                description: "a search engine enhanced with IA.".to_string(), 
                link: "https://www.google.com".to_string()
            }
        );
        v.push(
            Bookmark{
                id: "u-y-8-a".to_string(),
                name: "ChatGPT".to_string(), 
                description: "an IA platform".to_string(), 
                link: "https://www.chatgpt.com".to_string()
            }
        );
        let rows = //&self.bookmarks
        v.iter()
        .enumerate()
        .map(|(_i,r)| {
            Row::new()
            //.push(text(i.to_owned().to_string().as_str()))
            .push( text(r.id.clone()).width(100).height(50).color(Color::from_rgb(0.6, 0.3, 0.56)))
            .push( text(r.name.clone()))
            .push( text(r.description.clone()))
            .push( text(r.link.clone()))
            //.push(button("delete").on_press(move |_| Message::Remove((i))))
            .into()
        });
        Column::new().extend(rows.clone().into_iter()).push(text("Hello World!"))
    }

}
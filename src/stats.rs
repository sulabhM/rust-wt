use iced::{
    canvas::{self, Cache, Canvas, Cursor, Geometry, LineCap, Path, Stroke},
    executor, time, Application, Color, Command, Container, Element, Length,
    Point, Rectangle, Settings, Subscription,
};
use circular_queue::CircularQueue;

pub fn draw_stats() -> iced::Result {
    Stats::run(Settings {
        antialiasing: true,
        ..Settings::default()
    })
}

struct Stats {
    now: chrono::DateTime<chrono::Local>,
    stat_snapshot: Cache,
    stats_timeline: CircularQueue<u32>,
    max_stat: u32,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Tick(chrono::DateTime<chrono::Local>),
}

impl Application for Stats {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            Stats {
                now: chrono::Local::now(),
                stat_snapshot: Default::default(),
                stats_timeline: CircularQueue::with_capacity(200),
                max_stat: 0
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("WiredTiger Statistics")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Tick(local_time) => {
                let now = local_time;

                if now != self.now {
                    self.now = now;
                    self.stat_snapshot.clear();

                    let cur_stat = rand::random::<u32>();
                    self.stats_timeline.push(cur_stat);
                    if cur_stat > self.max_stat {
                        self.max_stat = cur_stat;
                    }
                }
            }
        }

        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        time::every(std::time::Duration::from_millis(500))
            .map(|_| Message::Tick(chrono::Local::now()))
    }

    fn view(&mut self) -> Element<Message> {
        let canvas = Canvas::new(self)
            .width(Length::Units(400))
            .height(Length::Units(400));

        Container::new(canvas)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(20)
            .center_x()
            .center_y()
            .into()
    }
}

impl canvas::Program<Message> for Stats {
    fn draw(&self, bounds: Rectangle, _cursor: Cursor) -> Vec<Geometry> {
        let stat_snapshot = self.stat_snapshot.draw(bounds.size(), |frame| {
            let stat_line_width = frame.width() / 200.0;
            let scale = (0.75 * frame.width()) / (self.max_stat as f32);

            //let center = frame.center();
            //let radius = frame.width().min(frame.height()) / 2.0;

            let background = Path::rectangle(Point::ORIGIN, frame.size());
            frame.fill(&background, Color::from_rgb8(161, 132, 47));

            let thin_stroke = Stroke {
                width: stat_line_width,
                color: Color::from_rgb8(82,29,29),
                line_cap: LineCap::Round,
                ..Stroke::default()
            };

            /* Draw a line for each stat point */
            for (i, stat) in self.stats_timeline.asc_iter().enumerate() {
                let x = (i as f32) * stat_line_width;
                let y1 = frame.height();
                let y2 = frame.height() - (scale * (*stat as f32));
                println!("{} plotting: {},{} : {},{}",i, x,y1,x,y2);

                let stat_line = Path::line(Point::new(x, y1), Point::new(x, y2));
                frame.with_save(|frame| {
                    frame.stroke(&stat_line, thin_stroke);
                })
            }
        });

        vec![stat_snapshot]
    }
}

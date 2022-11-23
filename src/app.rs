use tui::widgets::ListState;

const TABLES: [&str; 9] = [
    "Table1", "Table2", "Table3", "Table4", "Table5", "Table6", "Table7", "Table8", "Table9",
];

pub struct TabsState<'a> {
    pub titles: Vec<&'a str>,
    pub index: usize,
}

impl<'a> TabsState<'a> {
    pub fn new(titles: Vec<&'a str>) -> TabsState {
        TabsState { titles, index: 0 }
    }
    pub fn next(&mut self) {
        self.index = (self.index + 1) % self.titles.len();
    }

    pub fn previous(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        } else {
            self.index = self.titles.len() - 1;
        }
    }
}

pub struct StatefulList<T> {
    pub state: ListState,
    pub items: Vec<T>,
}

impl<T> StatefulList<T> {
    pub fn with_items(items: Vec<T>) -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items,
        }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
}

#[derive(Clone, Copy)]
pub enum Mode {
    FocusOnTab,
    FocusOnTable,
    FocusOnOperation,
    FocusOnItem,
}
enum ArrowKey {
    Up,
    Down,
    Left,
    Right,
}
pub struct ModeState {
    pub mode: Mode,
}
impl ModeState {
    pub fn new() -> ModeState {
        ModeState {
            mode: Mode::FocusOnTab,
        }
    }
    pub fn on_up(&mut self) {
        self.on_arrow_key(ArrowKey::Up)
    }

    pub fn on_down(&mut self) {
        self.on_arrow_key(ArrowKey::Down)
    }

    pub fn on_left(&mut self) {
        self.on_arrow_key(ArrowKey::Left)
    }

    pub fn on_right(&mut self) {
        self.on_arrow_key(ArrowKey::Right)
    }

    fn on_arrow_key(&mut self, arrow_key: ArrowKey) {
        match (self.mode, arrow_key) {
            (Mode::FocusOnTab, ArrowKey::Up) => {}
            (Mode::FocusOnTab, ArrowKey::Down) => self.mode = Mode::FocusOnTable,
            (Mode::FocusOnTab, ArrowKey::Left) => {}
            (Mode::FocusOnTab, ArrowKey::Right) => self.mode = Mode::FocusOnItem,

            (Mode::FocusOnTable, ArrowKey::Up) => self.mode = Mode::FocusOnTab,
            (Mode::FocusOnTable, ArrowKey::Down) => self.mode = Mode::FocusOnOperation,
            (Mode::FocusOnTable, ArrowKey::Left) => {}
            (Mode::FocusOnTable, ArrowKey::Right) => self.mode = Mode::FocusOnItem,

            (Mode::FocusOnOperation, ArrowKey::Up) => self.mode = Mode::FocusOnTable,
            (Mode::FocusOnOperation, ArrowKey::Down) => {}
            (Mode::FocusOnOperation, ArrowKey::Left) => {}
            (Mode::FocusOnOperation, ArrowKey::Right) => self.mode = Mode::FocusOnItem,

            (Mode::FocusOnItem, ArrowKey::Up) => {}
            (Mode::FocusOnItem, ArrowKey::Down) => {}
            (Mode::FocusOnItem, ArrowKey::Left) => self.mode = Mode::FocusOnTable,
            (Mode::FocusOnItem, ArrowKey::Right) => {}
        }
    }

    pub fn on_key(&mut self, c: char) {}
}

pub struct App<'a> {
    pub title: &'a str,
    pub should_quit: bool,
    pub tabs: TabsState<'a>,
    pub mode: ModeState,
    pub focus: bool,
    pub tables: StatefulList<&'a str>,
}

impl<'a> App<'a> {
    pub fn new(title: &'a str) -> App<'a> {
        App {
            title,
            should_quit: false,
            tabs: TabsState::new(vec!["Operation", "History"]),
            mode: ModeState::new(),
            focus: false,
            tables: StatefulList::with_items(TABLES.to_vec()),
        }
    }

    pub fn on_enter(&mut self) {
        self.focus = true
    }

    pub fn on_esc(&mut self) {
        self.focus = false
    }

    pub fn on_up(&mut self) {
        if !self.focus {
            self.mode.on_up();
            return;
        }
        match self.mode.mode {
            Mode::FocusOnTable => self.tables.previous(),
            _ => {}
        }
    }

    pub fn on_down(&mut self) {
        if !self.focus {
            self.mode.on_down();
            return;
        }
        match self.mode.mode {
            Mode::FocusOnTable => self.tables.next(),
            _ => {}
        }
    }

    pub fn on_left(&mut self) {
        if !self.focus {
            self.mode.on_left();
            return;
        }
        match self.mode.mode {
            Mode::FocusOnTab => self.tabs.previous(),
            _ => {}
        }
    }

    pub fn on_right(&mut self) {
        if !self.focus {
            self.mode.on_right();
            return;
        }
        match self.mode.mode {
            Mode::FocusOnTab => self.tabs.next(),
            _ => {}
        }
    }

    pub fn on_key(&mut self, c: char) {
        match c {
            'q' => {
                self.should_quit = true;
            }
            _ => {}
        }
    }
}

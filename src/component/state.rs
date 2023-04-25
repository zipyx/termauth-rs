use tui::widgets::ListState;

pub struct StateList<T> {
    pub state: ListState,
    pub items: Vec<T>,
}

impl <T>StateList<T> {
    pub fn all_items(items: Vec<T>) -> StateList<T> {
        StateList {
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

pub struct TabsState<'a> {
    pub titles: Vec<&'a str>,
    pub index: usize,
}

impl <'a>TabsState<'a> {
    pub fn new(titles: Vec<&str>) -> TabsState {
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

pub struct LoggedInState<'a> {
    pub titles: Vec<&'a str>,
    pub index: usize,
}

impl <'a>LoggedInState<'a> {
    pub fn new(titles: Vec<&str>) -> LoggedInState {
        LoggedInState { titles, index: 0 }
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

pub struct Server<'a> {
    pub name: &'a str,
    pub location: &'a str,
    pub coords: (f32, f32),
    pub status: &'a str,
}


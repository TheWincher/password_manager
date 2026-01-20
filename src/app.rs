use crossterm::event::{ self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers };
use ratatui::{
    DefaultTerminal,
    Frame,
    layout::{ Alignment, Constraint, Direction, Layout, Rect },
    style::{ Color, Style },
    widgets::{ Block, Borders, Clear, List, ListItem, ListState, Padding, Paragraph },
};
use serde::{ Deserialize, Serialize };
use std::env;
use std::{ fs::{ self, DirEntry, File }, io::{ self, Read, Write }, path::PathBuf };

use crate::vault::Vault;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AppState {
    AskCreateOrOpenVault,
    SelectExistingVault,
    AskMasterPassword,
    AskNewMasterPassword,
    VaultUncloked,
    VaultAddEntry,
    VaultShowEntry,
    VaultEditEntry,
    VaultDeleteEntry,
    Exit,
}

enum FileType {
    Config,
    Vault,
}

fn get_path(file_type: FileType) -> PathBuf {
    let file_name = match file_type {
        FileType::Config => "config.json",
        FileType::Vault => "vault.bin",
    };

    #[cfg(target_os = "windows")]
    {
        PathBuf::from(env::var("LOCALAPPDATA").expect("Could not get LOCALAPPDATA"))
            .join("PasswordManager")
            .join(file_name)
    }

    #[cfg(target_os = "macos")]
    {
        PathBuf::from(env::var("HOME").expect("Could not get HOME"))
            .join("Library")
            .join("Application Support")
            .join("PasswordManager")
            .join(file_name)
    }

    #[cfg(target_os = "linux")]
    {
        PathBuf::from(env::var("HOME").expect("Could not get HOME"))
            .join(".local")
            .join("share")
            .join("PasswordManager")
            .join(file_name)
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct Config {
    vault_path: PathBuf,
}

#[derive(Debug)]
pub struct App {
    state: AppState,
    input: InputState,
    ask_create_or_open_vault_state: AskCreateOrOpenVaultState,
    vault_selector_state: VaultSelectorState,
    vault_unlock_state: VaultUnclokedState,
    vault: Option<Vault>,
    config: Option<Config>,
}

impl App {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let config_path = get_path(FileType::Config);

        let (state, config) = if config_path.exists() {
            let mut file: File = File::open(config_path)?;

            let mut contents = String::new();
            file.read_to_string(&mut contents)?;

            let config: Config = serde_json::from_str(&contents)?;

            (AppState::AskMasterPassword, Some(config))
        } else {
            (AppState::AskCreateOrOpenVault, None)
        };

        Ok(App {
            input: InputState::new(),
            vault: None,
            state: state,
            config: config,
            ask_create_or_open_vault_state: AskCreateOrOpenVaultState::new(),
            vault_selector_state: VaultSelectorState::new(),
            vault_unlock_state: VaultUnclokedState::new(),
        })
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while self.state != AppState::Exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame) {
        let [top, center, bottom] = Layout::vertical([
            Constraint::Min(3),
            Constraint::Percentage(100),
            Constraint::Min(3),
        ]).areas(frame.area());

        let [center_left, center_right] = Layout::horizontal([
            Constraint::Percentage(33),
            Constraint::Fill(1),
        ]).areas(center);

        frame.render_widget(Block::bordered().title("top"), top);
        frame.render_widget(Block::bordered().title("center left"), center_left);
        frame.render_widget(Block::bordered().title("center right"), center_right);
        frame.render_widget(Block::bordered().title("bottom"), bottom);
        // match self.state {
        //     AppState::AskCreateOrOpenVault => {
        //         AskCreateOrOpenVault::draw(frame, &mut self.ask_create_or_open_vault_state);
        //     }
        //     AppState::SelectExistingVault => {
        //         VaultSelector::draw(frame, &mut self.vault_selector_state);
        //     }
        //     AppState::AskMasterPassword => {
        //         let rect = centered_rect(60, 5, frame.area());
        //         Input::draw(frame, rect, &self.input);
        //     }
        //     AppState::AskNewMasterPassword => {
        //         let rect = centered_rect(60, 5, frame.area());
        //         Input::draw(frame, rect, &self.input);
        //     }
        //     AppState::VaultUncloked => {}
        //     _ => {}
        // }
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(event) if event.kind == KeyEventKind::Press => {
                self.handle_key_event(event);
            }
            _ => {}
        }
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('c') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                self.state = AppState::Exit;
            }
            _ if self.state == AppState::AskCreateOrOpenVault =>
                self.ask_create_or_open_vault_state.on_key(key_event.code, &mut self.state),
            _ if self.state == AppState::SelectExistingVault =>
                self.vault_selector_state.on_key(key_event.code, &mut self.state),
            _ if
                self.state == AppState::AskMasterPassword ||
                self.state == AppState::AskNewMasterPassword
            => {
                self.input.on_key(
                    key_event.code,
                    &mut self.state,
                    &mut self.vault,
                    &self.vault_selector_state,
                    &mut self.config
                )
            }
            _ if self.state == AppState::VaultUncloked =>
                self.vault_unlock_state.on_key(key_event.code, &mut self.state),
            _ => {}
        }
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

#[derive(Debug)]
struct AskCreateOrOpenVaultState {
    list_state: ListState,
}

impl AskCreateOrOpenVaultState {
    fn new() -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));
        Self {
            list_state: list_state,
        }
    }

    pub fn on_key(&mut self, key: KeyCode, state: &mut AppState) {
        match key {
            KeyCode::Down | KeyCode::Char('j') => self.list_state.select_next(),
            KeyCode::Up | KeyCode::Char('k') => self.list_state.select_previous(),
            KeyCode::Enter | KeyCode::Right | KeyCode::Char('l') => self.enter(state),
            _ => {}
        }
    }

    fn enter(&mut self, state: &mut AppState) {
        match self.list_state.selected() {
            Some(0) => {
                *state = AppState::AskNewMasterPassword;
            }
            Some(1) => {
                *state = AppState::SelectExistingVault;
            }
            _ => {}
        }
    }
}

struct AskCreateOrOpenVault;
impl AskCreateOrOpenVault {
    fn draw(frame: &mut Frame, state: &mut AskCreateOrOpenVaultState) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // header
                Constraint::Min(5), // content
                Constraint::Length(2), // footer
            ])
            .split(frame.area());

        // Header
        frame.render_widget(
            Block::default().title(" Password Manager ─ Initialisation ").borders(Borders::ALL),
            layout[0]
        );

        // Content
        let items = vec![
            ListItem::new("Créer un nouveau vault"),
            ListItem::new("Ouvrir un vault existant")
        ];

        let list = List::new(items)
            .block(Block::default().borders(Borders::ALL))
            .highlight_symbol("▶ ")
            .highlight_style(Style::default().bg(Color::Blue));

        frame.render_stateful_widget(list, layout[1], &mut state.list_state);

        // Footer
        frame.render_widget(
            Paragraph::new("↑↓/jk Naviguer • Entrer Valider • q Quitter"),
            layout[2]
        );
    }
}

#[derive(Debug)]
struct VaultSelectorState {
    current_dir: PathBuf,
    entries: Vec<DirEntry>,
    list_state: ListState,
    file_selected: Option<PathBuf>,
}

impl VaultSelectorState {
    fn new() -> Self {
        let dir = std::env::current_dir().unwrap();
        let entries = Self::read_dir(&dir);
        let mut list_state = ListState::default();
        list_state.select(Some(0));

        Self {
            current_dir: dir,
            entries: entries,
            list_state: list_state,
            file_selected: None,
        }
    }

    fn read_dir(path: &PathBuf) -> Vec<DirEntry> {
        let mut entries: Vec<_> = fs
            ::read_dir(path)
            .unwrap()
            .filter_map(Result::ok)
            .filter(|item| {
                let file_type = item.file_type();
                if file_type.is_err() {
                    return false;
                }

                file_type.as_ref().unwrap().is_dir() ||
                    (file_type.unwrap().is_file() && item.file_name() == "vault.bin")
            })
            .collect();

        entries.sort_by_key(|e| e.path());
        entries
    }

    pub fn on_key(&mut self, key: KeyCode, state: &mut AppState) {
        match key {
            KeyCode::Down | KeyCode::Char('j') => self.list_state.select_next(),
            KeyCode::Up | KeyCode::Char('k') => self.list_state.select_previous(),
            KeyCode::Enter | KeyCode::Right | KeyCode::Char('l') => self.enter(state),
            KeyCode::Backspace | KeyCode::Char('g') => self.go_up(),
            _ => {}
        }
    }

    fn enter(&mut self, state: &mut AppState) {
        if let Some(i) = self.list_state.selected() {
            let path = self.entries[i].path();
            if path.is_dir() {
                self.current_dir = path;
                self.entries = Self::read_dir(&self.current_dir);
                self.list_state.select(Some(0));
            } else {
                self.file_selected = Some(path);
                *state = AppState::AskMasterPassword;
            }
        }
    }

    fn go_up(&mut self) {
        if let Some(parent) = self.current_dir.parent() {
            self.current_dir = parent.to_path_buf();
            self.entries = Self::read_dir(&self.current_dir);
            self.list_state.select(Some(0));
        }
    }
}

struct VaultSelector;
impl VaultSelector {
    fn draw(frame: &mut Frame, state: &mut VaultSelectorState) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // header
                Constraint::Min(5), // content
                Constraint::Length(2), // footer
            ])
            .split(frame.area());

        // Header
        frame.render_widget(
            Block::default().title(" Password Manager ─ Select Vault File").borders(Borders::ALL),
            layout[0]
        );

        let items: Vec<ListItem> = state.entries
            .iter()
            .map(|e| {
                let name = e.file_name().to_string_lossy().to_string();
                ListItem::new(name)
            })
            .collect();

        let list = List::new(items)
            .block(Block::default().title("Select vault file").borders(Borders::ALL))
            .highlight_style(Style::default().bg(Color::Blue))
            .highlight_symbol("> ");

        frame.render_stateful_widget(list, layout[1], &mut state.list_state);

        // Footer
        frame.render_widget(
            Paragraph::new("↑↓/jk Naviguer • Entrer Valider • q Quitter"),
            layout[2]
        );
    }
}

#[derive(Debug, Default)]
struct InputState {
    value: String,
    cursor: usize,
}

impl InputState {
    fn new() -> Self {
        Self {
            value: String::new(),
            cursor: 0,
        }
    }

    pub fn on_key(
        &mut self,
        key: KeyCode,
        state: &mut AppState,
        vault: &mut Option<Vault>,
        vault_selector_state: &VaultSelectorState,
        config: &mut Option<Config>
    ) {
        match key {
            KeyCode::Char(c) => {
                self.value.insert(self.cursor, c);
                self.cursor += 1;
            }
            KeyCode::Backspace => {
                if self.cursor > 0 {
                    self.cursor -= 1;
                    self.value.remove(self.cursor);
                }
            }
            KeyCode::Enter => {
                self.enter(state, vault, vault_selector_state, config);
            }
            _ => {}
        }
    }

    fn enter(
        &mut self,
        state: &mut AppState,
        vault: &mut Option<Vault>,
        vault_selector_state: &VaultSelectorState,
        config: &mut Option<Config>
    ) {
        let default_vault_path = get_path(FileType::Vault);
        let vault_path = vault_selector_state.file_selected.as_ref();
        if *state == AppState::AskMasterPassword {
            let v = Vault::open_existing(
                if vault_path.is_some() {
                    vault_path.unwrap()
                } else if config.is_some() {
                    &config.as_ref().unwrap().vault_path
                } else {
                    &default_vault_path
                },
                &self.value
            ).unwrap();

            *vault = Some(v);
            *state = AppState::VaultUncloked;
        } else if *state == AppState::AskNewMasterPassword {
            let v = Vault::new(&self.value).unwrap();
            *vault = Some(v);
            *state = AppState::VaultUncloked;
        }

        let binding = get_path(FileType::Vault);
        let vault_path_2 = vault_path.unwrap_or(&binding);
        self.value.clear();
        if config.is_none() {
            let new_config = Config {
                vault_path: vault_path_2.to_path_buf(),
            };
            *config = Some(new_config);
        } else {
            let mut old_config = config.take().unwrap();
            old_config.vault_path = vault_path_2.to_path_buf();
            *config = Some(old_config);
        }

        let config_path = get_path(FileType::Config);
        let file = File::create(config_path);

        match file {
            Ok(mut f) => {
                let contents = serde_json::to_string_pretty(config.as_ref().unwrap()).unwrap();
                f.write_all(contents.as_bytes()).unwrap();
            }
            Err(_) => {}
        }
    }
}

struct Input;
impl Input {
    fn draw(f: &mut Frame, area: Rect, input: &InputState) {
        // Important : Clear efface le fond
        f.render_widget(Clear, area);

        let block = Block::default()
            .title("Master password")
            .borders(Borders::ALL)
            .padding(Padding::horizontal(1));
        let paragraph = Paragraph::new("*".repeat(input.value.len()))
            .block(block)
            .alignment(Alignment::Left);

        f.render_widget(paragraph, area);
        // Position du curseur
        let x = area.x + (input.cursor as u16) + 2;
        let y = area.y + 1;

        f.set_cursor_position((x, y));
    }
}

#[derive(Debug)]
struct VaultUnclokedState {
    list_state: ListState,
}

impl VaultUnclokedState {
    fn new() -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));
        Self { list_state }
    }

    fn on_key(&mut self, key: KeyCode, state: &mut AppState) {
        match key {
            KeyCode::Down | KeyCode::Char('j') => self.list_state.select_next(),
            KeyCode::Up | KeyCode::Char('k') => self.list_state.select_previous(),
            KeyCode::Char('a') => {
                *state = AppState::VaultAddEntry;
            }
            //KeyCode::Enter | KeyCode::Right | KeyCode::Char('l') => self.enter(state),
            _ => {}
        }
    }
}

struct VaultUncloked;
impl VaultUncloked {
    fn draw(frame: &mut Frame, state: &mut VaultUnclokedState, vault: &mut Vault) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // header
                Constraint::Min(5), // content
                Constraint::Length(2), // footer
            ])
            .split(frame.area());

        // Header
        frame.render_widget(
            Block::default().title(" Password Manager ─ Vault").borders(Borders::ALL),
            layout[0]
        );

        let items: Vec<ListItem> = vault
            .get_entries()
            .iter()
            .map(|e| ListItem::new(e.service.clone()))
            .collect();

        let list = List::new(items)
            .block(Block::default().title("Entries").borders(Borders::ALL))
            .highlight_style(Style::default().bg(Color::Blue))
            .highlight_symbol("> ");

        frame.render_stateful_widget(list, layout[1], &mut state.list_state);

        // Footer
        frame.render_widget(
            Paragraph::new("↑↓/jk Naviguer • Entrer Valider • q Quitter • a Ajouter"),
            layout[2]
        );
    }
}

#[derive(Debug)]
struct VaultAddEntry;
impl VaultAddEntry {
    fn draw(frame: &mut Frame, state: &mut VaultUnclokedState, vault: &mut Vault) {
        let rect = centered_rect(60, 60, frame.area());
        frame.render_widget(Clear, rect);

        let block = Block::default()
            .title("Ajouter une entrée")
            .borders(Borders::ALL)
            .padding(Padding::horizontal(1));

        frame.render_widget(block, rect);

        let rec_1 = centered_rect(90, 5, rect);
        let block = Block::default()
            .title("Service")
            .borders(Borders::ALL)
            .padding(Padding::horizontal(1));

        let paragraph = Paragraph::new("test").block(block).alignment(Alignment::Left);

        frame.render_widget(paragraph, rec_1);
    }
}

#[derive(Debug)]
struct VaultShowEntry;
impl VaultShowEntry {
    fn draw(frame: &mut Frame, state: &mut VaultUnclokedState, vault: &mut Vault) {}
}

#[derive(Debug)]
struct VaultEditEntry;
impl VaultEditEntry {
    fn draw(frame: &mut Frame, state: &mut VaultUnclokedState, vault: &mut Vault) {}
}

#[derive(Debug)]
struct VaultDeleteEntry;
impl VaultDeleteEntry {
    fn draw(frame: &mut Frame, state: &mut VaultUnclokedState, vault: &mut Vault) {}
}

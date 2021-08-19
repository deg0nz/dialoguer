use std::{io, ops::Rem};

use crate::theme::{SimpleTheme, TermThemeRenderer, Theme};
use crate::paging::Paging;

use console::{Key, Term};

/// Renders a select prompt.
///
/// User can select from one or more options.
/// Interaction returns index of an item selected in the order they appear in `item` invocation or `items` slice.
///
/// ## Examples
///
/// ```rust,no_run
/// use dialoguer::{
///     Select,
///     theme::ColorfulTheme
/// };
/// use console::Term;
///
/// fn main() -> std::io::Result<()> {
///     let items = vec!["Item 1", "item 2"];
///     let selection = Select::with_theme(&ColorfulTheme::default())
///         .items(&items)
///         .default(0)
///         .interact_on_opt(&Term::stderr())?;
///
///     match selection {
///         Some(index) => println!("User selected item : {}", items[index]),
///         None => println!("User did not select anything")
///     }
///
///     Ok(())
/// }
/// ```
pub struct Select<'a> {
    default: usize,
    items: Vec<String>,
    prompt: Option<String>,
    clear: bool,
    theme: &'a dyn Theme,
    paged: bool,
}

impl<'a> Default for Select<'a> {
    fn default() -> Select<'a> {
        Select::new()
    }
}

impl<'a> Select<'a> {
    /// Creates a select prompt builder with default theme.
    pub fn new() -> Select<'static> {
        Select::with_theme(&SimpleTheme)
    }

    /// Creates a select prompt builder with a specific theme.
    ///
    /// ## Examples
    /// ```rust,no_run
    /// use dialoguer::{
    ///     Select,
    ///     theme::ColorfulTheme
    /// };
    ///
    /// fn main() -> std::io::Result<()> {
    ///     let selection = Select::with_theme(&ColorfulTheme::default())
    ///         .item("Option A")
    ///         .item("Option B")
    ///         .interact()?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn with_theme(theme: &'a dyn Theme) -> Select<'a> {
        Select {
            default: !0,
            items: vec![],
            prompt: None,
            clear: true,
            theme,
            paged: false,
        }
    }

    /// Enables or disables paging
    ///
    /// Paging is disabled by default
    #[deprecated(since="0.9.0", note="Activating paging has no effect anymore. Paging will be activated automatically if needed.")]
    pub fn paged(&mut self, val: bool) -> &mut Select<'a> {
        self.paged = val;
        self
    }

    /// Indicates whether select menu should be erased from the screen after interaction.
    ///
    /// The default is to clear the menu.
    pub fn clear(&mut self, val: bool) -> &mut Select<'a> {
        self.clear = val;
        self
    }

    /// Sets initial selected element when select menu is rendered
    ///
    /// Element is indicated by the index at which it appears in `item` method invocation or `items` slice.
    pub fn default(&mut self, val: usize) -> &mut Select<'a> {
        self.default = val;
        self
    }

    /// Add a single item to the selector.
    ///
    /// ## Examples
    /// ```rust,no_run
    /// use dialoguer::Select;
    ///
    /// fn main() -> std::io::Result<()> {
    ///     let selection: usize = Select::new()
    ///         .item("Item 1")
    ///         .item("Item 2")
    ///         .interact()?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn item<T: ToString>(&mut self, item: T) -> &mut Select<'a> {
        self.items.push(item.to_string());
        self
    }

    /// Adds multiple items to the selector.
    ///
    /// ## Examples
    /// ```rust,no_run
    /// use dialoguer::Select;
    ///
    /// fn main() -> std::io::Result<()> {
    ///     let items = vec!["Item 1", "Item 2"];
    ///     let selection: usize = Select::new()
    ///         .items(&items)
    ///         .interact()?;
    ///
    ///     println!("{}", items[selection]);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn items<T: ToString>(&mut self, items: &[T]) -> &mut Select<'a> {
        for item in items {
            self.items.push(item.to_string());
        }
        self
    }

    /// Sets the select prompt.
    ///
    /// When a prompt is set the system also prints out a confirmation after
    /// the selection.
    ///
    /// ## Examples
    /// ```rust,no_run
    /// use dialoguer::Select;
    ///
    /// fn main() -> std::io::Result<()> {
    ///     let selection = Select::new()
    ///         .with_prompt("Which option do you prefer?")
    ///         .item("Option A")
    ///         .item("Option B")
    ///         .interact()?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn with_prompt<S: Into<String>>(&mut self, prompt: S) -> &mut Select<'a> {
        self.prompt = Some(prompt.into());
        self
    }

    /// Enables user interaction and returns the result.
    ///
    /// Similar to [interact_on](#method.interact_on) except for the fact that it does not allow selection of the terminal.
    /// The dialog is rendered on stderr.
    /// Result contains index of a selected item.
    pub fn interact(&self) -> io::Result<usize> {
        self.interact_on(&Term::stderr())
    }

    /// Enables user interaction and returns the result.
    ///
    /// This method is similar to [interact_on_opt](#method.interact_on_opt) except for the fact that it does not allow selection of the terminal.
    /// The dialog is rendered on stderr.
    /// Result contains `Some(index)` if user selected one of items or `None` if user cancelled with 'Esc' or 'q'.
    pub fn interact_opt(&self) -> io::Result<Option<usize>> {
        self.interact_on_opt(&Term::stderr())
    }

    /// Like [interact](#method.interact) but allows a specific terminal to be set.
    ///
    /// ## Examples
    ///```rust,no_run
    /// use dialoguer::Select;
    /// use console::Term;
    ///
    /// fn main() -> std::io::Result<()> {
    ///     let selection = Select::new()
    ///         .item("Option A")
    ///         .item("Option B")
    ///         .interact_on(&Term::stderr())?;
    ///
    ///     println!("User selected option at index {}", selection);
    ///
    ///     Ok(())
    /// }
    ///```
    pub fn interact_on(&self, term: &Term) -> io::Result<usize> {
        self._interact_on(term, false)?
            .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Quit not allowed in this case"))
    }

    /// Like [interact_opt](#method.interact_opt) but allows a specific terminal to be set.
    ///
    /// ## Examples
    /// ```rust,no_run
    /// use dialoguer::Select;
    /// use console::Term;
    ///
    /// fn main() -> std::io::Result<()> {
    ///     let selection = Select::new()
    ///         .item("Option A")
    ///         .item("Option B")
    ///         .interact_on_opt(&Term::stdout())?;
    ///
    ///     match selection {
    ///         Some(position) => println!("User selected option at index {}", position),
    ///         None => println!("User did not select anything")
    ///     }
    ///
    ///     Ok(())
    /// }
    /// ```
    #[inline]
    pub fn interact_on_opt(&self, term: &Term) -> io::Result<Option<usize>> {
        self._interact_on(term, true)
    }

    /// Like `interact` but allows a specific terminal to be set.
    fn _interact_on(&self, term: &Term, allow_quit: bool) -> io::Result<Option<usize>> {
        let mut paging = Paging::new(term, &self.items);

        if self.items.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Empty list of items given to `Select`",
            ));
        }

        let mut render = TermThemeRenderer::new(term, self.theme);
        let mut sel = self.default;

        if let Some(ref prompt) = self.prompt {
            render.select_prompt(prompt)?;
        }

        let mut size_vec = Vec::new();

        for items in self
            .items
            .iter()
            .flat_map(|i| i.split('\n'))
            .collect::<Vec<_>>()
        {
            let size = &items.len();
            size_vec.push(*size);
        }

        term.hide_cursor()?;

        loop {
            paging.update(sel)?;

            // This should go somewhere else
            // We also need to handle the following case:
            // Paging was active, terminal is resized to a size where paging is disabled
            // -> (Unpaged) Prompt must be written at the top of the screen

            if paging.enabled() {
                // This may be redundant to last statement in loop
                // But is needed to prevent the prompt to be written multiple times
                term.clear_last_lines(paging.capacity())?;

                if let Some(ref prompt) = self.prompt {
                    render.select_prompt_paged(prompt, paging.current_page() + 1, paging.pages())?;
                }
            }

            paging.render_page_items(|idx, item| render.select_prompt_item(item, sel == idx))?;

            match term.read_key()? {
                Key::ArrowDown | Key::Char('j') => {
                    if sel == !0 {
                        sel = 0;
                    } else {
                        sel = (sel as u64 + 1).rem(self.items.len() as u64) as usize;
                    }
                }
                Key::Escape | Key::Char('q') => {
                    if allow_quit {
                        if self.clear {
                            term.clear_last_lines(self.items.len())?;
                            term.show_cursor()?;
                            term.flush()?;
                        }

                        return Ok(None);
                    }
                }
                Key::ArrowUp | Key::Char('k') => {
                    if sel == !0 {
                        sel = self.items.len() - 1;
                    } else {
                        sel = ((sel as i64 - 1 + self.items.len() as i64)
                            % (self.items.len() as i64)) as usize;
                    }
                }
                Key::ArrowLeft | Key::Char('h') => {
                    if paging.enabled() {
                        sel = paging.previous_page();
                    }
                }
                Key::ArrowRight | Key::Char('l') => {
                    if paging.enabled() {
                        sel = paging.next_page();
                    }
                }

                Key::Enter | Key::Char(' ') if sel != !0 => {
                    if self.clear {
                        render.clear()?;
                    }

                    if let Some(ref prompt) = self.prompt {
                        render.select_prompt_selection(prompt, &self.items[sel])?;
                    }

                    term.show_cursor()?;
                    term.flush()?;

                    return Ok(Some(sel));
                }
                _ => {}
            }

            render.clear_preserve_prompt(&size_vec)?;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_str() {
        let selections = &[
            "Ice Cream",
            "Vanilla Cupcake",
            "Chocolate Muffin",
            "A Pile of sweet, sweet mustard",
        ];

        assert_eq!(
            Select::new().default(0).items(&selections[..]).items,
            selections
        );
    }

    #[test]
    fn test_string() {
        let selections = vec!["a".to_string(), "b".to_string()];

        assert_eq!(
            Select::new().default(0).items(&selections[..]).items,
            selections
        );
    }

    #[test]
    fn test_ref_str() {
        let a = "a";
        let b = "b";

        let selections = &[a, b];

        assert_eq!(
            Select::new().default(0).items(&selections[..]).items,
            selections
        );
    }
}

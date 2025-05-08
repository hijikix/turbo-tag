use std::collections::HashMap;

use egui::Id;

#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum Anchor {
    List,
    Detail,
    Debug,
}

impl Anchor {
    #[allow(dead_code)]
    fn all() -> Vec<Self> {
        vec![Self::List, Self::Detail, Self::Debug]
    }

    #[allow(dead_code)]
    fn from_str_case_insensitive(anchor: &str) -> Option<Self> {
        let anchor = anchor.to_lowercase();
        Self::all().into_iter().find(|x| x.to_string() == anchor)
    }
}

impl std::fmt::Display for Anchor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut name = format!("{self:?}");
        name.make_ascii_lowercase();
        f.write_str(&name)
    }
}

impl From<Anchor> for egui::WidgetText {
    fn from(value: Anchor) -> Self {
        Self::RichText(egui::RichText::new(value.to_string()))
    }
}

impl Default for Anchor {
    fn default() -> Self {
        Self::Debug
    }
}

#[derive(Clone, Default)]
pub struct NextPage {
    pub anchor: Anchor,
    pub params: HashMap<String, String>,
}

/// ID used for storing the next page in egui's memory
const NEXT_PAGE_ID: &str = "next_page";

/// Store the next page to navigate to in egui's temporary memory
pub fn move_to_page(ctx: &egui::Context, next_page: NextPage) {
    ctx.memory_mut(|mem| {
        mem.data.insert_temp(Id::from(NEXT_PAGE_ID), next_page);
    });
}

/// Retrieve and remove the next page from egui's temporary memory
pub fn get_next_page(ctx: &egui::Context) -> Option<NextPage> {
    ctx.memory_mut(|mem| {
        let id = Id::from(NEXT_PAGE_ID);
        let next_page = mem.data.get_temp(id);
        if next_page.is_some() {
            mem.data.remove_temp::<NextPage>(id);
        }
        next_page
    })
}

pub trait AppPage {
    fn on_move_page(&mut self, ctx: &egui::Context, params: &HashMap<String, String>);
}

// Define a new trait that combines eframe::App and AppPage
pub trait AppWithPage: eframe::App + AppPage {}

// Implement the trait for all types that implement both eframe::App and AppPage
impl<T: eframe::App + AppPage> AppWithPage for T {}

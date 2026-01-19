use gpui::{IntoElement, ParentElement};
use ui::{Banner, prelude::*};

#[derive(IntoElement)]
pub struct YoungAccountBanner;

impl RenderOnce for YoungAccountBanner {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        // Мы просто возвращаем пустой div, чтобы баннер ничего не рисовал
        div() 
    }
}

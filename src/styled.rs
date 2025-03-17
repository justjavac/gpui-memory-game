use crate::colors::*;
use gpui::Styled;

pub trait StyledExt: Styled {
  fn text_indigo_100(self) -> Self {
    self.text_color(indigo_100())
  }

  fn text_indigo_200(self) -> Self {
    self.text_color(indigo_200())
  }

  fn text_indigo_300(self) -> Self {
    self.text_color(indigo_300())
  }

  fn border_indigo_600(self) -> Self {
    self.border_color(indigo_600())
  }

  fn border_indigo_500(self) -> Self {
    self.border_color(indigo_500())
  }

  fn border_indigo_700(self) -> Self {
    self.border_color(indigo_700())
  }

  fn border_indigo_800(self) -> Self {
    self.border_color(indigo_800())
  }

  fn bg_indigo_900(self) -> Self {
    self.bg(indigo_900())
  }

  fn bg_indigo_950(self) -> Self {
    self.bg(indigo_950())
  }
}

impl<E: Styled> StyledExt for E {}

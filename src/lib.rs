// plotters-iced
//
// Iced backend for Plotters
// Copyright: 2021, Joylei <leingliu@gmail.com>
// License: MIT

/*!
   The Plotters Iced backend.

   This is an implementation of a Iced backend for Plotters.

   This backend has been optimized as for speed. Note that some specific plotting features supported in the Bitmap backend may not be implemented there, though.

   See the examples for more details.

   ## Example
   ```rust,ignore
   struct MyChart;
   impl Chart<Message> for MyChart {
      fn build_chart<DB:DrawingBackend>(&self, builder: ChartBuilder<DB>) {
         //build your chart here, please refer to plotters for more details
      }
   }

   impl MyChart {
      fn view(&mut self)->Element<Message> {
         ChartWidget::new(self)
         .width(Length::Unit(200))
               .height(Length::Unit(200))
               .into()
      }
   }
   ```

*/

extern crate iced_graphics;
extern crate iced_native;
extern crate plotters;
pub extern crate plotters_backend;

mod backend;
mod chart;
mod error;
mod triangulate;
mod utils;

pub use chart::Chart;
pub use chart::ChartWidget;
pub use plotters::chart::ChartBuilder;
pub use plotters_backend::DrawingBackend;

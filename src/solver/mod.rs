pub mod cubic;
pub mod linear;
pub mod linear_system;
pub mod quadratic;
pub mod quartic;
pub mod quintic;

pub use cubic::{solve_cubic, solve_cubic_equation};
pub use linear::{solve_linear, solve_linear_equation};
pub use linear_system::{solve_2x2_system, solve_3x3_system, solve_linear_system};
pub use quadratic::{solve_quadratic, solve_quadratic_equation};
pub use quartic::{solve_quartic, solve_quartic_equation};
pub use quintic::{solve_quintic, solve_quintic_equation};
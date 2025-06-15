pub mod constant;
pub mod stock_price;

use jiff::civil::Date;

pub trait Observable<T>: Clone + Send + Sync {
    fn value(&self, t: Date) -> T;
}

impl Observable<f64> for f64 {
    fn value(&self, _t: Date) -> f64 {
        *self
    }
}

impl Observable<bool> for bool {
    fn value(&self, _t: Date) -> bool {
        *self
    }
}

pub type ObservableF64 = Box<dyn Observable<f64>>;
pub type ObservableBool = Box<dyn Observable<bool>>;

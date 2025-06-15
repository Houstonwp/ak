use jiff::civil::Date;

use crate::observable::Observable;

#[derive(Clone, Copy, Debug)]
pub enum Currency {
    USD,
    EUR,
    GBP,
    JPY,
    CNY,
}

#[derive(Clone, Copy, Debug)]
pub struct CashFlow<O: Observable<f64>> {
    pub date: Date,
    pub amount: O,
    pub currency: Currency,
}

pub enum Contract {
    Zero,
    Pay {
        date: Date,
        amount: f64,
        currency: Currency,
    },
}

pub struct ContractBuilder<O: Observable<f64>> {
    pub cash_flows: Vec<CashFlow<O>>,
}

impl<O: Observable<f64>> ContractBuilder<O> {
    pub fn new() -> Self {
        ContractBuilder {
            cash_flows: Vec::new(),
        }
    }

    pub fn add_cash_flow(&mut self, date: Date, amount: O, currency: Currency) {
        self.cash_flows.push(CashFlow {
            date,
            amount,
            currency,
        });
    }

    pub fn build(self) -> Contract {
        if self.cash_flows.is_empty() {
            Contract::Zero
        } else {
            // For simplicity, we just return the first cash flow as a Pay contract.
            let first = &self.cash_flows[0];
            Contract::Pay {
                date: first.date,
                amount: first.amount.value(first.date),
                currency: first.currency,
            }
        }
    }
}

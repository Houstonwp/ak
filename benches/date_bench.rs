use ak::{Date, Frequency, generate_cashflow_dates};
use criterion::{Criterion, black_box, criterion_group, criterion_main};

fn bench_cashflow_dates(c: &mut Criterion) {
    let start = Date::new(2023, 1, 31).expect("valid date");

    c.bench_function("cashflow_dates_monthly_360", |b| {
        b.iter(|| {
            let dates =
                generate_cashflow_dates(black_box(start), black_box(360), Frequency::Monthly)
                    .expect("date generation");
            black_box(dates.len());
        });
    });

    c.bench_function("cashflow_dates_weekly_2600", |b| {
        b.iter(|| {
            let dates =
                generate_cashflow_dates(black_box(start), black_box(2600), Frequency::Weekly)
                    .expect("date generation");
            black_box(dates.len());
        });
    });
}

criterion_group!(benches, bench_cashflow_dates);
criterion_main!(benches);

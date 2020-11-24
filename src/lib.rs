enum Polynom {
    Empty,
    Full {
        coefficient: f64,
        exponent: i32,
        next: Box<Polynom>,
    },
}

impl std::fmt::Display for Polynom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Polynom::Empty => Ok(()),
            Polynom::Full {
                coefficient,
                exponent,
                next,
            } => {
                write!(f, "{}", coefficient)?;
                match exponent {
                    0 => {}
                    1 => write!(f, "x")?,
                    _ => write!(f, "x^{}", exponent)?,
                }
                if let Polynom::Full { coefficient, .. } = **next {
                    if coefficient < 0. {
                        write!(f, " {}", next)?;
                    } else {
                        write!(f, " + {}", next)?;
                    }
                }
                Ok(())
            }
        }
    }
}

impl Polynom {
    fn eval(&self, x: f64) -> f64 {
        match self {
            Polynom::Empty => 0.,
            Polynom::Full {
                coefficient,
                exponent,
                next,
            } => coefficient * x.powi(*exponent) + next.eval(x),
        }
    }

    fn differentiate(&self) -> Polynom {
        match self {
            Polynom::Empty => Polynom::Empty,
            Polynom::Full {
                coefficient,
                exponent,
                next,
            } => {
                let coefficient = coefficient * *exponent as f64;
                if coefficient == 0. {
                    next.differentiate()
                } else {
                    Polynom::Full {
                        coefficient,
                        exponent: exponent - 1,
                        next: Box::new(next.differentiate()),
                    }
                }
            }
        }
    }

    fn find_root(&self, guess: f64) -> f64 {
        fn find_root_impl(poly: &Polynom, derivative: &Polynom, guess: f64) -> f64 {
            // println!("find_root_impl({}, {}, {})", poly, derivative, guess);
            let next_guess = guess - poly.eval(guess) / derivative.eval(guess);
            if (next_guess - guess).abs() < 0.000001 {
                next_guess
            } else {
                find_root_impl(poly, derivative, next_guess)
            }
        }
        let derivative = self.differentiate();
        find_root_impl(self, &derivative, guess)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    const PRECISION: f64 = 0.000001;

    #[test]
    fn find_root_exercise_sheet_first_test() {
        // given
        let under_test = Polynom::Full {
            coefficient: 1.,
            exponent: 3,
            next: Box::new(Polynom::Full {
                coefficient: -2.,
                exponent: 2,
                next: Box::new(Polynom::Full {
                    coefficient: -11.,
                    exponent: 1,
                    next: Box::new(Polynom::Full {
                        coefficient: 12.,
                        exponent: 0,
                        next: Box::new(Polynom::Empty),
                    }),
                }),
            }),
        };
        // when:
        let actual = under_test.find_root(-4.);
        // then:
        assert_approx_eq!(actual, -3., PRECISION);

        // when:
        let actual = under_test.find_root(0.);
        // then:
        assert_approx_eq!(actual, 1., PRECISION);

        // when:
        let actual = under_test.find_root(2.35287527);
        // then:
        assert_approx_eq!(actual, 4., PRECISION);
    }

    #[test]
    fn find_root_exercise_sheet_second_test() {
        // given
        let under_test = Polynom::Full {
            coefficient: 1.,
            exponent: 3,
            next: Box::new(Polynom::Full {
                coefficient: -2.,
                exponent: 2,
                next: Box::new(Polynom::Full {
                    coefficient: -5.,
                    exponent: 1,
                    next: Box::new(Polynom::Full {
                        coefficient: 6.,
                        exponent: 0,
                        next: Box::new(Polynom::Empty),
                    }),
                }),
            }),
        };
        // when:
        let actual = under_test.find_root(-3.);
        // then:
        assert_approx_eq!(actual, -2., PRECISION);

        // when:
        let actual = under_test.find_root(0.);
        // then:
        assert_approx_eq!(actual, 1., PRECISION);

        // when:
        let actual = under_test.find_root(4.);
        // then:
        assert_approx_eq!(actual, 3., PRECISION);
    }

    #[test]
    fn find_root_exercise_sheet_third_test() {
        // given
        let under_test = Polynom::Full {
            coefficient: 2.,
            exponent: 4,
            next: Box::new(Polynom::Full {
                coefficient: 7.,
                exponent: 3,
                next: Box::new(Polynom::Full {
                    coefficient: 6.,
                    exponent: 2,
                    next: Box::new(Polynom::Full {
                        coefficient: 8.,
                        exponent: 1,
                        next: Box::new(Polynom::Full {
                            coefficient: 12.,
                            exponent: 0,
                            next: Box::new(Polynom::Empty),
                        }),
                    }),
                }),
            }),
        };
        // when:
        let actual = under_test.find_root(0.);
        // then:
        assert_approx_eq!(actual, -1.5, PRECISION);

        // when:
        let actual = under_test.find_root(2.5);
        // then:
        assert_approx_eq!(actual, -2.5943, PRECISION);
        // TEST IS WRONG!!!
    }

    #[test]
    fn print_empty_polynom() {
        // given:
        let empty_poly = Polynom::Empty;

        // when:
        let actual = empty_poly.to_string();

        // then:
        assert_eq!(actual, "");
    }

    #[test]
    fn print_polynoms() {
        // given:
        let under_test = Polynom::Full {
            coefficient: 1.,
            exponent: 3,
            next: Box::new(Polynom::Full {
                coefficient: 2.,
                exponent: 2,
                next: Box::new(Polynom::Full {
                    coefficient: -11.,
                    exponent: 1,
                    next: Box::new(Polynom::Full {
                        coefficient: 12.,
                        exponent: 0,
                        next: Box::new(Polynom::Empty),
                    }),
                }),
            }),
        };

        // when:
        let actual = under_test.to_string();

        // then:
        assert_eq!(actual, "1x^3 + 2x^2 -11x + 12");
    }

    #[test]
    fn eval_polynoms() {
        // given:
        let under_test = Polynom::Full {
            coefficient: -5.,
            exponent: 3,
            next: Box::new(Polynom::Full {
                coefficient: 2.,
                exponent: 2,
                next: Box::new(Polynom::Full {
                    coefficient: -4.,
                    exponent: 1,
                    next: Box::new(Polynom::Full {
                        coefficient: 12.,
                        exponent: 0,
                        next: Box::new(Polynom::Empty),
                    }),
                }),
            }),
        };

        // when:
        let actual = under_test.eval(2.);
        // then:
        assert_eq!(actual, -28.);

        // when:
        let actual = under_test.eval(0.);
        // then:
        assert_eq!(actual, 12.);
    }

    #[test]
    fn differentiate_polynoms() {
        // given:
        let under_test = Polynom::Full {
            coefficient: -1.,
            exponent: 3,
            next: Box::new(Polynom::Full {
                coefficient: 2.,
                exponent: 2,
                next: Box::new(Polynom::Full {
                    coefficient: -11.,
                    exponent: 1,
                    next: Box::new(Polynom::Full {
                        coefficient: 12.,
                        exponent: 0,
                        next: Box::new(Polynom::Empty),
                    }),
                }),
            }),
        };

        // when:
        let actual = under_test.differentiate();

        // then:
        assert_eq!(actual.to_string(), "-3x^2 + 4x -11");
    }
}

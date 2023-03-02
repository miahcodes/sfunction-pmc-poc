use shopify_function::prelude::*;
use shopify_function::Result;

use serde::{Serialize};

// Use the shopify_function crate to generate structs for the function input and output
generate_types!(
    query_path = "./input.graphql",
    schema_path = "./schema.graphql"
);

// Use the shopify_function crate to declare your function entrypoint
#[shopify_function]
fn function(input: input::ResponseData) -> Result<output::FunctionResult> {
    let no_discount = output::FunctionResult {
        discounts: vec![],
        discount_application_strategy: output::DiscountApplicationStrategy::FIRST,
    };

    // Iterate all the lines in the cart to create discount targets
    let targets = input.cart.lines
        .iter()
        // Only include cart lines with a quantity higher than two
        // .filter(|line| line.quantity >= 2)
        // Only include cart lines with a targetable product variant
        .filter_map(|line| match &line.merchandise {
            input::InputCartLinesMerchandise::ProductVariant(variant) => Some(variant),
            input::InputCartLinesMerchandise::CustomProduct => None,
        })
        // Use the variant ID to create a discount target
        .map(|variant| output::Target {
            product_variant: Some(output::ProductVariantTarget {
                id: variant.id.to_string(),
                quantity: None,
            })
        })
        .collect::<Vec<output::Target>>();

    // Get the value of the input cart's "straight_payments" attribute and return early if it's true
    let straight_payments = match input.cart.attribute {
        Some(attr) => match attr.value {
            Some(value) => value,
            None => {
                eprintln!("No value found for 'straight_payments' attribute");
                return Ok(no_discount);
            }
        },
        None => {
            eprintln!("No value found for 'straight_payments' attribute");
            return Ok(no_discount);
        }
    };

    if straight_payments == "false" {
        // You can use STDERR for debug logs in your function
        eprintln!("User is paying with straight payments, so we're not hiding any payment methods");
        return Ok(no_discount);
    }

    // The shopify_function crate serializes your function result and writes it to STDOUT
    Ok(output::FunctionResult {
        discounts: vec![output::Discount {
            message: None,
            // Apply the discount to the collected targets
            targets,
            // Define a percentage-based discount
            value: output::Value {
                fixed_amount: None,
                percentage: Some(output::Percentage {
                    value: "10.0".to_string()
                })
            }
        }],
        discount_application_strategy: output::DiscountApplicationStrategy::FIRST,
    })
}

#[cfg(test)]
mod tests;

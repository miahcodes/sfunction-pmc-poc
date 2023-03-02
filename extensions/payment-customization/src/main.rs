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
    let no_changes = output::FunctionResult { operations: vec![] };

    // Get the value of the input cart's "straight_payments" attribute and return early if it's true
    let straight_payments = match input.cart.attribute {
        Some(attr) => match attr.value {
            Some(value) => value,
            None => {
                eprintln!("No value found for 'straight_payments' attribute");
                return Ok(no_changes);
            }
        },
        None => {
            eprintln!("No value found for 'straight_payments' attribute");
            return Ok(no_changes);
        }
    };

    if straight_payments == "false" {
        // You can use STDERR for debug logs in your function
        eprintln!("User is paying with straight payments, so we're not hiding any payment methods");
        return Ok(no_changes);
    }

    // Find the payment method to hide, and create a hide output operation from it
    // (this will be None if not found)
    let hide_payment_method = input.payment_methods
        .iter()
        .find(|&method| method.name.contains(&"Pay by Installments".to_string()))
        .map(|method| output::HideOperation {
            payment_method_id: method.id.to_string()
        });

    // The shopify_function crate serializes your function result and writes it to STDOUT
    Ok(output::FunctionResult { operations: vec![
        output::Operation {
            hide: hide_payment_method,
            move_: None,
            rename: None
        }
    ] })
}

#[cfg(test)]
mod tests;

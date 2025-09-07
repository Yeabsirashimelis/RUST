/*
Let's model a road trip!

Define a `start_trip` function that creates and returns
a String of "The plan is..."

Invoke the `start_trip` function in `main` and save its
return value to a `trip` variable.

We want to pass the String to three separate functions
that will mutate the String without transferring ownership.

Define a `visit_philadelphia` function that concatenates
the text "Philadephia" to the end of the String. Invoke
the function in `main`. Then, invoke `push_str` on the String
to concatenate the content " and " to the end. Mak sure to
include the spaces.

Define a `visit_new_york` function that concatenates the
text "New York" to the end of the String. Invoke the function
in `main`. Repeat the previous logic to concatenate " and "
to the end of the String.

Define a `visit_boston` function that concatenates the
text "Boston." to the end of the String. Invoke the function
in `main`. Concatenate a period to the end of the
String/sentence.

Define a `show_itinerary` function that will print out
the final version of the String. Find a way to do so
without transferring ownership.

Invoke `show_itinerary`. The final output should be:

"The plan is...Philadelphia and New York and Boston."
*/

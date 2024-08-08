# Weather CLI App in Rust

This Rust CLI application fetches and displays weather information for a specified city using the OpenWeatherMap API. It retrieves details like temperature, humidity, pressure, and wind speed, and presents them in a color-coded format based on the weather description. 

How It Works
------------

1. The user inputs a city and country code.
2. The app fetches the weather data from OpenWeatherMap API.
3. It then displays the weather information in a color-coded format based on the weather description.

Code Breakdown
--------------

### Structs and `#[derive()]`

The application uses Rust's `#[derive()]` attribute to automatically generate implementations for certain traits. Here’s how they are used:

* `#[derive(Deserialize, Debug)]` on structs (`WeatherResponse`, `Weather`, `Main`, `Wind`) allows them to be deserialized from JSON and printed for debugging.

### Dependencies

In `Cargo.toml`, the following dependencies are used:

* **`reqwest`**: An HTTP client library. We use the `blocking` feature to perform synchronous HTTP requests.
* **`serde`**: Provides serialization and deserialization support. The `derive` feature is used to automatically generate code for converting JSON into Rust structs.
* **`serde_json`**: Provides JSON parsing capabilities.
* **`colored`**: For colorizing terminal output.

### Result<T, E>

`Result<T, E>` is a Rust enum used for error handling. It represents either a success (`Ok(T)`) or an error (`Err(E)`). In the `get_weather_info` function, `Result<WeatherResponse, reqwest::Error>` indicates that the function returns a `WeatherResponse` on success or a `reqwest::Error` on failure.

### Use of `?`

The `?` operator simplifies error handling by automatically propagating errors. It is used in `get_weather_info` to return early if an error occurs while making the HTTP request or parsing the JSON response.

### `reqwest::blocking::get`

* **`reqwest::blocking::get`**: This function performs a blocking HTTP GET request, meaning it waits for the server response before continuing execution.
* **Blocking vs. Async**: `reqwest` supports both synchronous (`blocking`) and asynchronous operations. In this app, synchronous operations are used for simplicity and ease of understanding.

### Functions

The `get_weather_info` function retrieves weather data from the OpenWeatherMap API. It takes the city name, country code, and API key as parameters and returns a `Result`. If the API call is successful, it returns the weather data wrapped in an `Ok` variant. If something goes wrong, it returns an error in an `Err` variant. The function constructs a URL, performs a blocking HTTP GET request using `reqwest`, and parses the JSON response. Any errors encountered during this process are propagated using the `?` operator.

The `display_weather_info` function takes a `WeatherResponse` object and formats it for display. It extracts relevant details such as weather description, temperature, humidity, pressure, and wind speed. The output is color-coded based on the weather description using the `colored` crate, which makes the text visually distinct for different weather conditions. Finally, it prints the formatted weather information to the terminal.

The `main` function is the core of the application. It starts by greeting the user and asking for input. It calls `get_weather_info` to fetch the weather data and then uses `display_weather_info` to show it. If there is an error, it prints an error message. The function loops, allowing the user to search for another city's weather, and exits when the user decides to stop.

Running the App
---------------

1. Ensure you have Rust installed.
2. Clone or download the repository.
3. Run `cargo build` to compile the app.
4. Execute `cargo run` to start the application.

Feel free to modify the API key and explore different cities and countries!



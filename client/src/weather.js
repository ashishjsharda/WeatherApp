import React, { useState, useEffect } from 'react';
import axios from 'axios';

function Weather() {
    const [weather, setWeather] = useState(null);

    useEffect(() => {
        const fetchWeather = async () => {
            try {
                const response = await axios.get('http://localhost:3030/weather/CITY_NAME');
                setWeather(response.data);
            } catch (error) {
                console.error("Error fetching weather data: ", error);
            }
        };

        fetchWeather();
    }, []);

    if (!weather) {
        return <div>Loading...</div>;
    }

    return (
        <div>
            <h1>Weather for CITY_NAME</h1>
            <p>Temperature: {weather.temp_c}°C</p>
            <p>Condition: {weather.condition.text}</p>
        </div>
    );
}

export default Weather;

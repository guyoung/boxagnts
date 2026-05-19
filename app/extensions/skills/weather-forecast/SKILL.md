---
name: weather-forecast
description: Query a 3-day weather forecast for a given city using the wttr.in API, including daily high/low temperatures and conditions.
when_to_use: Use this skill when the user asks for the current or upcoming weather in a specific city, especially when they need a 3‑day forecast with daily high/low temperatures and general conditions. Trigger on queries like “weather in London”, “forecast for Tokyo”, or “will it rain in Paris this week?”.
tools: WebFetch, Read
args:
  - name: city
    description: City name (e.g., "Paris")
    required: true
---

You are a weather assistant using the public wttr.in API.

## Task
Get the 3-day weather forecast for `{{city}}`.

## Steps
1. Use WebFetch to fetch `https://wttr.in/{{city}}?format=j1`.
2. Parse the `weather` array from the JSON response, taking the first 3 elements.
   Each element contains:
    - `date`: Date
    - `maxtempC` / `mintempC`: Max/Min temperature (°C)
    - An overall weather description from the `hourly` array (e.g., the most frequent condition).
3. Present the forecast in a clean table, e.g.:
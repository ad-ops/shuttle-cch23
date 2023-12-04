POST http://localhost:8000/4/strength
Content-Type: application/json
[
    { "name": "Dasher", "strength": 5 },
    { "name": "Dancer", "strength": 6 },
    { "name": "Prancer", "strength": 4 },
    { "name": "Vixen", "strength": 7 }
]
HTTP 200
`22`

POST http://localhost:8000/4/contest
Content-Type: application/json
[
    {
      "name": "Dasher",
      "strength": 5,
      "speed": 50.4,
      "height": 80,
      "antler_width": 36,
      "snow_magic_power": 9001,
      "favorite_food": "hay",
      "cAnD13s_3ATeN-yesT3rdAy": 2
    },
    {
      "name": "Dancer",
      "strength": 6,
      "speed": 48.2,
      "height": 65,
      "antler_width": 37,
      "snow_magic_power": 4004,
      "favorite_food": "grass",
      "cAnD13s_3ATeN-yesT3rdAy": 5
    }
]
HTTP 200
[Asserts]
jsonpath "$.fastest" contains "Dasher"
jsonpath "$.tallest" contains "Dasher"
jsonpath "$.magician" contains "Dasher"
jsonpath "$.consumer" contains "Dancer"
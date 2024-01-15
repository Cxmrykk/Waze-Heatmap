### Tools
- [Leaflet Heat](https://github.com/Leaflet/Leaflet.heat/)
- [Leaflet](https://leafletjs.com/)

### Server Dependencies
- `libsqlite3-dev`
- `default-libmysqlclient-dev`
- `libpq-dev`

### Response:
- `alerts`: [200 maximum]
- `endTimeMillis`: number
- `startTimeMIllis`: number
- `startTime`: string
- `endTime`: string 

### Request (URL) Parameters:
> https://www.waze.com/live-map/api/georss
- `top`: -33.8463184046000
- `bottom`: -33.89429422015962
- `left`: 151.1265563964844
- `right`: 151.3243103027344
- `env`: row
- `types`: alerts, traffic, users 

### Precision
- Waze has 6 trailing digits of decimal coordinate precision (OSM Standard - https://wiki.openstreetmap.org/wiki/Precision_of_coordinates)

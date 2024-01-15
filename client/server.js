const sqlite3 = require('sqlite3').verbose();
const express = require('express');
const path = require('path');

let db = new sqlite3.Database('./db.sqlite', sqlite3.OPEN_READONLY, (err) => {
  if (err) {
    console.error(err.message);
  }
  console.log('Connected to the SQLite database.');
});

let app = express();
app.use(express.static(path.join(__dirname, 'public')));
app.use('/node_modules', express.static(path.join(__dirname, 'node_modules')));

app.get('/data', (req, res) => {
  let sql = `SELECT x, y FROM alerts`;
  db.all(sql, [], (err, rows) => {
    if (err) {
      throw err;
    }
    res.json(rows);
  });
});

app.listen(3000, () => {
  console.log('Server is running on port 3000');
});


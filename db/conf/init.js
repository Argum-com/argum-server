argumDB = db.getSiblingDB('dev');
db.createCollection('rooms', { capped: false });
let roomData = {
    "id": "string",
    "name": "string",
    "isActive": true,
    "messages": [
      {
        "timestamp": "Date",
        "text": "string",
        "author": "string"
      },
      {
        "timestamp": "Date",
        "text": "string",
        "author": "string"
      }
    ]
  };  
db.rooms.insert(roomData);

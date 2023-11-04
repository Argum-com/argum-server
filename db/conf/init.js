argumDB = db.getSiblingDB('dev');
db.createCollection('rooms', { capped: false });
let roomData = {
    "name": "string",
    "isActive": true,
    "messages": [
      {
        "timestamp": "Date1",
        "text": "string",
        "author": "65455f29ec879711b3fb9448"
      },
      {
        "timestamp": "Date2",
        "text": "string",
        "author": "65455f29ec879711b3fb9455"
      }
    ]
  };  
db.rooms.insert(roomData);

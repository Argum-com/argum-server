argumDB = db.getSiblingDB("dev");
db.createCollection("rooms", { capped: false });
let roomData = {
  name: "string",
  isActive: true,
  messages: [
    {
      timestamp: NumberLong(1699091507820),
      text: "text 1",
      author: { $oid: "654562acd869fedb026f724e" },
    },
    {
      timestamp: NumberLong(1699091507820),
      text: "text 2",
      author: { $oid: "654562acd869fedb026f734e" },
    },
  ],
};
db.rooms.insert(roomData);

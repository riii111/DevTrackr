print(
  "Start #################################################################"
);

const adminDB = db.getSiblingDB("admin");
adminDB.auth(
  process.env.MONGO_INITDB_ROOT_USERNAME,
  process.env.MONGO_INITDB_ROOT_PASSWORD
);

const dbName = process.env.MONGO_INITDB_DATABASE;
const user = process.env.MONGO_INITDB_ROOT_USERNAME;
const password = process.env.MONGO_INITDB_ROOT_PASSWORD;

db = db.getSiblingDB(dbName);

if (db.getUser(user) == null) {
  db.createUser({
    user: user,
    pwd: password,
    roles: [{ role: "readWrite", db: dbName }],
  });
  print("User created");
} else {
  print("User already exists");
}

print("END #################################################################");

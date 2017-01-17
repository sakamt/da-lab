
from pandas.io import sql
from collections import UserDict
import sqlite3


class DB(object):

    def __init__(self, dbpath):
        self.conn = sqlite3.connect(dbpath)

    def _read_table(self, table_name):
        return sql.read_sql("SELECT * FROM {}".format(table_name), self.conn)

    def truth(self):
        return self._read_table("truth").set_index("id")

    def observation(self):
        return self._read_table("observation").set_index("id")

    def enkf(self):
        return self._read_table("enkf").set_index("id")

    def get_truth(self, id):
        tbname = self.truth().ix[id]["table_name"]
        return self._read_table(tbname).set_index("time")

    def get_observation(self, id):
        tbname = self.observation().ix[id]["table_name"]
        return self._read_table(tbname).set_index("time")

    def get_enkf(self, id):
        data = self.enkf().ix[id]
        return EnKF(self, data)


class EnKF(UserDict):

    def __init__(self, db, data):
        super().__init__(data)
        self._db = db

    def stat(self):
        return self._db._read_table(self["stat_table"]).set_index("time").sort_index()

    def ensemble_iter(self):
        df = self._db._read_table(self["ensemble_table"]).set_index("time").sort_index()
        for t, row in df.iterrows():
            b = self._read_table(row["forecasted"])
            a = self._read_table(row["analysized"])
            yield (t, b, a)

    def truth(self):
        return self._db.get_truth(self["truth_id"])

    def observation(self):
        return self._db.get_truth(self["observation_id"])

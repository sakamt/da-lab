
from pandas.io import sql
import sqlite3


class Handler(object):

    def __init__(self, dbpath):
        self.conn = sqlite3.connect(dbpath)

    def _read_table(self, table_name):
        return sql.read_sql("SELECT * FROM {}".format(table_name), self.conn)

    def truth(self):
        return self._read_table("truth").set_index("id")

    def observation(self):
        return self._read_table("observation").set_index("id")

    def ensemble(self):
        return self._read_table("ensemble").set_index("id")

    def enkf(self):
        return self._read_table("enkf").set_index("id")

    def get_truth(self, id):
        tbname = self.truth().ix[id]["table_name"]
        return self._read_table(tbname).set_index("time")

    def get_observation(self, id):
        tbname = self.observation().ix[id]["table_name"]
        return self._read_table(tbname).set_index("time")

    def ensemble_iter(self, id):
        tbname = self.ensemble().ix[id]["table_name"]
        df = self._read_table(tbname).set_index("time").sort_index()
        for t, row in df.iterrows():
            b = self._read_table(row["forecasted"])
            a = self._read_table(row["analysized"])
            yield (t, b, a)

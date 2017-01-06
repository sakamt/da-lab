
from pandas.io import sql
import sqlite3


class Handler(object):

    def __init__(self, dbpath):
        self.conn = sqlite3.connect(dbpath)

    def read_table(self, table_name):
        return sql.read_sql("SELECT * FROM {}".format(table_name), self.conn)

    def truth(self):
        return self.read_table("truth").set_index("id")

    def observation(self):
        return self.read_table("observation").set_index("id")

    def ensemble(self):
        return self.read_table("ensemble").set_index("id")

    def enkf(self):
        return self.read_table("enkf").set_index("id")

    def get_truth(self, id):
        tbname = self.truth().ix[id]["table_name"]
        return self.read_table(tbname).set_index("time")

    def get_observation(self, id):
        tbname = self.observation().ix[id]["table_name"]
        return self.read_table(tbname).set_index("time")

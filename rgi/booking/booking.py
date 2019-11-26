#!/usr/bin/env python

from sqlalchemy.ext.automap import automap_base
from sqlalchemy.orm import Session
from sqlalchemy import create_engine
import sys
import json
import os
import re


Base = automap_base()
engine = create_engine(os.getenv("DATABASE_URL"))
Base.prepare(engine, reflect=True)
Booking = Base.classes.booking
session = Session(engine)


def get(data):
    """
    Get data from the database
    :param data: {id}
    :return: Booking dictionary or {error: (True/False)}
    """
    results = session.query(Booking).filter(Booking.id == data["args"]["id"]).all()
    if len(results) == 1:
        result = {}
        for attr, value in k.__dict__.items():
            result[attr] = value
        return json.dump(result)

    else:
        return json.dump({"error": "Dawid Kubis to rozbil"})


def post(data):
    """
    Adds new data to db
    :param data: Booking dictionary by it's id
    :return: {success: (True/"error message")}
    """

    results = session.query(Booking).filter(Booking.id == data.id).all()
    if len(results) == 0:
        result = Booking()
        for key, value in data.items():
            result.key = value
        session.add(result)
        session.commit()
        return json.dump({"success": True})
    else:
        return json.dump({"success": "Do you want to kill it?"})

def patch(data):
    """
    Update data in the database
    :param data: Booking dictionary
    :return: {success: (True/"error message")}
    """

    results = session.query(Booking).filter(Booking.id == data.id).all()
    if len(results) == 1:
        result = results[0]
        for key, value in data.items():
            result.key = value
        session.add(result)
        session.commit()
        return json.dump({"success": True})
    else:
        return json.dump({"success": "blame David Kubis for this one"})

def delete(data):
    """
    Deletes event by it's id
    :param data: {id}
    :return: {success: (True/False)}
    """

    results = session.query(Booking).filter(Booking.id == data.id).all()
    if len(results) == 1:
        session.delete(results[0])
        return json.dump({"success": True})
    else:
        return json.dump({"success": "Delete failed, bitches"})



methods = {"get": get, "post": post, "patch": patch, "delete": delete}
txt = sys.stdin.read()
txt = re.sub(",[ \t\r\n]+}", "}", txt)
txt = re.sub(",[ \t\r\n]+\]", "]", txt)
data = json.loads(txt)
if len(sys.argv) < 2:
    methods["get"](data)
else:
    methods[sys.argv[1].lower()](data)


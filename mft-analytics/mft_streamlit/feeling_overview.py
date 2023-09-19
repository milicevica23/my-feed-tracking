import streamlit as st
import pandas as pd
import duckdb 

st.write("Here's our first attempt at using data to create a table:")

con = duckdb.connect("C:\\Users\\A108587977\\git\\my-projects\\my-feed-tracking\\mft-analytics\\duck_db")

df = con.sql("SELECT * FROM user").fetchdf()
print(type(df))
df
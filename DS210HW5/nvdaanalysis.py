import pandas as pd
import numpy as np
import matplotlib.pyplot as plt

df = pd.read_csv('NVDA.csv')
print(df.describe())

df['Date'] = pd.to_datetime(df['Date'])
df = df.sort_values(by="Date")

start_date = df.iloc[0]['Date']
end_date = df.iloc[-1]['Date']
start_price = df.iloc[0]['Close']
end_price = df.iloc[-1]['Close']

num_years = (end_date - start_date).days / 365.25

cagr = ((end_price / start_price) ** (1 / num_years) - 1) * 100

print(f"Average Annual Growth (CAGR): {cagr:.2f}%")
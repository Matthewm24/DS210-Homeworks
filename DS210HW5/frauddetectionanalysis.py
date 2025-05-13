import matplotlib.pyplot as plt
import numpy as np
import pandas as pd


df = pd.read_csv('Fraud Detection Dataset.csv')
print(df.head())
print(df.describe())

df['Time_of_Transaction'] = pd.to_numeric(df['Time_of_Transaction'], errors='coerce')
most_common_hour = df[df['Fraudulent'] == 1]['Time_of_Transaction'].mode()[0]
print("Most common time for fraudulent activity:", most_common_hour)
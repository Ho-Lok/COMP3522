# import pandas as pd
# import numpy as np
# import matplotlib.pyplot as plt
# from sklearn.linear_model import LinearRegression
# from sklearn.preprocessing import MinMaxScaler

# # Load the data
# df = pd.read_csv("row_attribute_counts.csv")

# # Initialize the scaler
# scaler = MinMaxScaler()

# # Adjust and normalize the data
# num = df["NumOfStops"] * np.where(df["isCircular"], 0.5, 1)
# distance = df["distance"] * np.where(df["isCircular"], 0.5, 1)

# fare = df["Fare"]
# fare_norm = scaler.fit_transform(df[["Fare"]])
# NumOfStops_norm = scaler.fit_transform(num.values.reshape(-1, 1))
# distance_norm = scaler.fit_transform(distance.values.reshape(-1, 1))

# # Create a scatter plot
# plt.figure(figsize=(10, 6))
# dot_size = 10
# plt.scatter(NumOfStops_norm, fare, color="blue", marker='o', s=dot_size, label='Number of Stops')
# plt.scatter(distance_norm, fare, color="red", marker='o', s=dot_size, label='Distance')

# # Fit linear regression models
# model_stops = LinearRegression()
# model_stops.fit(NumOfStops_norm, fare)

# model_distance = LinearRegression()
# model_distance.fit(distance_norm, fare)

# # Predict values for the regression lines
# x_fit_stops = np.linspace(NumOfStops_norm.min(), NumOfStops_norm.max(), 100).reshape(-1, 1)
# y_fit_stops = model_stops.predict(x_fit_stops)
# x_fit_distance = np.linspace(distance_norm.min(), distance_norm.max(), 100).reshape(-1, 1)
# y_fit_distance = model_distance.predict(x_fit_distance)

# # Calculate the standard error of the predictions
# y_pred_stops = model_stops.predict(NumOfStops_norm)
# residuals_stops = NumOfStops_norm.flatten() - y_pred_stops
# std_error_stops = np.std(residuals_stops)

# y_pred_distance = model_distance.predict(distance_norm)
# residuals_distance = distance_norm.flatten() - y_pred_distance
# std_error_distance = np.std(residuals_distance)

# # Plot the regression lines
# plt.plot(x_fit_stops, y_fit_stops, color='blue', linewidth=3, label='Regression Line for Number of Stops')
# plt.plot(x_fit_distance, y_fit_distance, color='red', linewidth=3, label='Regression Line for Distance')

# # Add variation range for number of stops
# plt.fill_between(x_fit_stops.flatten(), 
#                  y_fit_stops.flatten() - 1.96 * std_error_stops, 
#                  y_fit_stops.flatten() + 1.96 * std_error_stops, 
#                  color='blue', alpha=0.2)

# # Add variation range for distance
# plt.fill_between(x_fit_distance.flatten(), 
#                  y_fit_distance.flatten() - 1.96 * std_error_distance, 
#                  y_fit_distance.flatten() + 1.96 * std_error_distance, 
#                  color='red', alpha=0.2)

# # Add titles and labels
# plt.title('Sample Scatter Plot with Regression Lines and Variation Ranges', fontsize=24)
# plt.ylabel('Fare', fontsize=16)
# plt.xlabel('Normalized Values', fontsize=16)

# # Add a legend
# plt.legend(fontsize=16)

# # Show grid
# plt.grid(True)

# # Show the plot
# plt.show()

import pandas as pd
import matplotlib.pyplot as plt

# Load the data
df = pd.read_csv("row_attribute_counts.csv")

# Filter groups with less than 2 entries
filtered_df = df.groupby("NumOfStops").filter(lambda x: len(x) >= 5)

# Calculate mean, max, min, Q1, and Q3 fare for each number of stops
grouped = filtered_df.groupby("NumOfStops")["Fare"].agg(['mean', 'max', 'min']).reset_index()
grouped['Q1'] = filtered_df.groupby("NumOfStops")["Fare"].quantile(0.25).values
grouped['Q3'] = filtered_df.groupby("NumOfStops")["Fare"].quantile(0.75).values

# Create the plot
plt.figure(figsize=(10, 6))

# Plot mean fare
plt.plot(grouped["NumOfStops"], grouped["mean"], color='blue', label='Mean Fare', linewidth=2)

# Fill between Q1 and Q3 for IQR
plt.fill_between(grouped["NumOfStops"], grouped["Q1"], grouped["Q3"], color='lightblue', alpha=0.8, label='IQR (Q1 to Q3)')
# Fill between max and min fare
plt.fill_between(grouped["NumOfStops"], grouped["max"], grouped["min"], color='lightblue', alpha=0.25, label='Max/Min Fare Range')

# Add titles and labels
plt.title('Number of Stops vs. Fare with Mean and Range', fontsize=20)
plt.xlabel('Number of Stops', fontsize=16)
plt.ylabel('Fare', fontsize=16)
plt.legend(fontsize=16)
plt.grid(True)

# Show the plot
plt.show()
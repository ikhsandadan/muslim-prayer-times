# **Muslim Prayer Times Project**

For the StackUp August Coding Challenge, I have created an application to assist Muslim worshippers in performing their religious duties more conveniently.

This project is built using Rust as the backend and NextJS with [Tauri](https://tauri.app/) as the frontend. I will provide a detailed explanation of each page:


![1](https://github.com/user-attachments/assets/46f133e5-8d82-49e4-ab98-f101992dc90a)


## **1**. **Home**

   On this page, the user can see the current date and the Islamic calendar date. It also displays the nearest upcoming prayer time based on the user's location, along with a translation of a verse from the Quran. Additionally, there is a button for the user to indicate whether they have performed the prayer at the scheduled time.


![2](https://github.com/user-attachments/assets/5627e19d-65e6-4cb3-8ae0-afa063f26d99)

![3](https://github.com/user-attachments/assets/c18dd382-24e0-4fc2-85ca-361729622680)


## **2.** **Quran**

   This page allows the user to view the entire Quran, including all surahs and verses, with their translations. Users can also listen to each verse as desired.


![6](https://github.com/user-attachments/assets/009d352f-de2e-43c4-a20f-94b4a4aa9d13)

![4](https://github.com/user-attachments/assets/e78aae88-7954-42a5-8b29-64d574d17972)

![5](https://github.com/user-attachments/assets/ed065c51-6c9a-4fa8-aaa2-61016233ee92)


## **3.** **Calendar**

   On this page, the user can view the Gregorian calendar and check their prayer record for each day. It also displays Islamic holidays and events.


![7](https://github.com/user-attachments/assets/ac3ffb96-9f90-4ea3-b624-176ae991f482)

![8](https://github.com/user-attachments/assets/5c8b1331-8a4a-4786-9f7d-c07ff045aac4)

![9](https://github.com/user-attachments/assets/c664b2ad-57c0-4031-b06d-4069a6f037c6)

## **4.** **Statistics**

   This page presents all the user's prayer records stored in the database, which can be viewed according to the user's preferences, such as today, yesterday, this week, last week, this month, last month, or a custom date range.



The entire backend of this application is built using Rust, and it is called by NextJS using Tauri. For example:


![kode](https://github.com/user-attachments/assets/c3da3a5a-c992-4ed6-abbd-61816472270f)


To display the prayer record chart on the Statistics page, I have also used Rust to draw heatmap charts using SVG format.

The full Rust code can be viewed at this [link](https://github.com/ikhsandadan/muslim-prayer-times/tree/main/src-tauri/src).

This application can be fully executed on Windows, as the output is in the ".exe" format, which can be downloaded from this [link](https://github.com/ikhsandadan/muslim-prayer-times/releases/tag/app).

Finally, here is a demo video showcasing the usage of this application: 


[![muslim-prayer-times-demo](https://img.youtube.com/vi/r55FrQ3CvtU/0.jpg)](https://www.youtube.com/watch?v=r55FrQ3CvtU)

## Balto.AI Telegram

To run the project:

```shell
RUST_LOG=trace TELOXIDE_TOKEN=secret_token cargo run 
```

### Algorithm

The general algorithm used by the bot is pretty straightforward:

```

Starting Dialogue:

1.  Prompt to the user the following question: "Which is your role?"

2.  If the user clicks 'Employer', go to the 'Employer Dialogue', otherwise, if they
    clicks 'Employee', go to 'Employee Dialogue'


Employer Dialogue:

1.  The employer can choose to get the list of the employees to get previous reports. Or,
    they can wait for a report from some employee.

2.  When a report is received in PDF, the Employer can rate the overall produced work.


Employee Dialogue:

1.  The employee has just one command 'Start Send Report'. 

2.  When an employee send many audio/text/image, those will be sent to the backend to generate
    a bullet point list containing a summarize of the media contents.

3.  If the user finished uploading the content, they can press one of the following buttons:
        a.  The 'Finish' button: the content will be send to the model's backend to generate a summary
            to be sent to the employee for a confirmation. Go to the point (4).
        b.  The 'Abort' button: the sending operation will be abort. Go to point (1).

4.  The backend will send a bullet point list containing all the information summarized:
        a.  If the summarized bullet points satisfy the employee, then it can proceed
            with the report's sending pressing the 'Send' button.
        b.  Otherwise, the user can abort using the 'Abort' button. Go to point (1).
        
5.  The employee will get a notification if they sent the report. The loop can restart again.
```

The user's locale is extracted by Telegram's settings. So, it's not necessary to input the 
user's language to the chat. This project is thought having functionality in mind, focusing
on the pratical usage, since people nowadays are used with chat applications.
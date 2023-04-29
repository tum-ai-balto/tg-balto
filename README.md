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
1.  The employee doesn't have any command available.
2.  When an employee send an audio/text/image, this will be sent to the backend to generate
    a bullet point containing a summarize of the media content.
        a.  If the summarized bullet points satisfy the employee, then it can proceed
            with the report's sending. 
        b.  Otherwise, restart the procedure going back to point 1.
3.  The employee will get a notification if they sent the report. The loop can restart again.
```

The user's locale is extracted by Telegram's settings. So, it's not necessary to input the 
user's language to the chat. This project is thought having functionality in mind, focusing
on the pratical usage, since people nowadays are used with chat applications.
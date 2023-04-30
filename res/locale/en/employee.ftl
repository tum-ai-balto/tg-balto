select-employer = <b>Write the name of the employer you want to apply for</b>

    <i>To see the list of available employers, use the command <code>{ get-employers-cmd }</code>.</i>

received-employer = <b>Thank you for setting up the employer!</b>{ send-media }

send-media = As employee, you can send me media content (<b>text</b>, <b>audio</b>, <b>photo</b>) to generate a report to send.

    <b><i>When you are done sending the material, use the command <code>{ send-report-cmd }</code>.</i></b>
    <i>To see the list of available employers, use the command <code>{ get-employers-cmd }</code>.</i>
send-report-cmd = /send
get-employers-cmd = /employers

unrecognized-message = This message is not valid. Please send a text, an audio or an image.

sent-media = We are sending your content to the AI. The summary will be coming soon.

summary-generated = This is what you sent: <i>{$reportSummary}</i>

    If you're ok with it, press <b>{ send }</b> button.
send = Send 📬

sent-report = Message sent! Now you can use /reset to restart the process, or make a new report and /send
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <meta http-equiv="X-UA-Compatible" content="IE=edge">
    <title>Your Payment Confirmation</title>
    <style type="text/css">
        body, table, td, a { -webkit-text-size-adjust: 100%; -ms-text-size-adjust: 100%; }
        table, td { mso-table-lspace: 0pt; mso-table-rspace: 0pt; }
        img { -ms-interpolation-mode: bicubic; border: 0; height: auto; line-height: 100%; outline: none; text-decoration: none; }
        table { border-collapse: collapse !important; }
        body { height: 100% !important; margin: 0 !important; padding: 0 !important; width: 100% !important; background-color: #f4f4f4; }

        body {
            font-family: 'Helvetica Neue', Helvetica, Arial, sans-serif;
        }

        a {
            color: #007bff;
            text-decoration: underline;
        }

        .wrapper {
            width: 100%;
            max-width: 600px;
            margin: 0 auto;
            background-color: #ffffff;
        }

        .header {
            padding: 20px 30px;
            text-align: center;
            border-bottom: 1px solid #eaeaea;
        }
        .header img {
            max-width: 180px;
        }

        .content {
            padding: 30px 30px;
            line-height: 1.6;
            color: #333333;
            font-size: 16px;
        }

        .content h1 {
            font-size: 24px;
            font-weight: 600;
            margin-top: 0;
            margin-bottom: 20px;
            color: #111111;
        }

        .order-summary {
            width: 100%;
            margin-top: 25px;
            margin-bottom: 25px;
            border: 1px solid #dddddd;
        }
        .order-summary th, .order-summary td {
            text-align: left;
            padding: 12px;
            font-size: 14px;
        }
        .order-summary th {
            background-color: #f8f8f8;
            font-weight: 600;
            border-bottom: 1px solid #dddddd;
        }
        .order-summary td.label {
            font-weight: bold;
            width: 35%;
        }
        .order-summary td.total {
            font-weight: bold;
            font-size: 16px;
            border-top: 1px solid #dddddd;
        }

        .button-td, .button-a {
            display: block;
            text-align: center;
        }
        .button-a {
            background-color: #007bff;
            border-radius: 5px;
            color: #ffffff;
            display: inline-block;
            font-size: 16px;
            font-weight: bold;
            padding: 15px 30px;
            text-decoration: none;
            margin-top: 20px;
            margin-bottom: 20px;
        }

        .footer-container {
            padding: 0 0 20px 0;
        }
        .footer-div {
            font-family: 'Helvetica Neue',Helvetica,Arial,sans-serif;
            box-sizing: border-box;
            font-size: 14px;
            width: 100%;
            clear: both;
            color: #999;
            margin: 0;
            padding: 20px;
            background-color: #f6f6f6;
        }
        .footer-div td {
             font-family: 'Helvetica Neue',Helvetica,Arial,sans-serif;
             box-sizing: border-box;
             font-size: 12px;
             vertical-align: top;
             color: #999;
             text-align: center;
             margin: 0;
             padding: 0 0 20px;
        }
         .footer-div a {
            color: #999;
            text-decoration: underline;
         }

        @media screen and (max-width: 600px) {
            .wrapper {
                width: 100% !important;
                max-width: 100% !important;
            }
            .content, .header {
                padding: 20px !important;
            }
            .order-summary td.label {
                width: auto !important;
            }
            .footer-div {
                 padding: 20px !important;
            }
        }

    </style>
</head>
<body>
    <center style="width: 100%; background-color: #f4f4f4;">

        <table role="presentation" class="wrapper" border="0" cellpadding="0" cellspacing="0" align="center">

            <tr>
                <td class="header">
                    <a href="{{ website_link }}" target="_blank">
                        <img src="{{ logo_url }}" alt="{{ company_name }} Logo" width="180">
                    </a>
                </td>
            </tr>


            <tr>
                <td class="content">
                    <h1>Payment Successful!</h1>

                    <p>Hi {{ customer_name }},</p>

                    <p>Thank you for your purchase! We've successfully processed your payment for your {{ company_name }} plan.</p>

                    <p>Here's a summary of your transaction:</p>


<table role="presentation" class="order-summary" border="0" cellpadding="0" cellspacing="0">
                        <tr>
                            <td class="label">Item:</td>
                            <td>{{ plan_name }}</td>
                        </tr>
                         <!-- NEW: Display plan details -->
                         <tr>
                            <td class="label">Models Included:</td>
                            <td>{{ plan_model_amount }}</td>
                        </tr>
                         <tr>
                            <td class="label">Credits Included:</td>
                            <td>{{ plan_credit_amount }}</td>
                        </tr>
                         <!-- End of NEW -->
                         <tr>
                            <td class="label">Transaction Date:</td>
                            <td>{{ transaction_date }}</td> <!-- Includes time now -->
                        </tr>
                         <tr>
                            <td class="label">Transaction ID:</td>
                            <td>{{ transaction_id }}</td>
                        </tr>
                         <tr>
                            <td class="label total">Amount Paid:</td>
                            <td class="total">{{ amount_paid }}</td> <!-- Uses new format -->
                        </tr>
                    </table>

                    <p>Your {{ plan_name }} plan has been applied to your account. Head over to your dashboard and start generating amazing images:</p>


                    <table role="presentation" border="0" cellpadding="0" cellspacing="0" width="100%">
                         <tr>
                            <td align="center" style="padding: 20px 0;">
                                <table role="presentation" border="0" cellpadding="0" cellspacing="0">
                                    <tr>
                                        <td class="button-td">
                                            <a href="{{ dashboard_link }}" class="button-a" target="_blank">Go to Your Dashboard</a>
                                        </td>
                                     </tr>
                                </table>
                             </td>
                        </tr>
                    </table>

                    <p>Thanks again for choosing {{ company_name }}!</p>

                    <p>Sincerely,<br>The {{ company_name }} Team</p>
                </td>
            </tr>


            <tr>
                <td class="footer-container">

                </td>
            </tr>

        </table>


        <div class="footer-div">
             <table width="100%" style="border-collapse: collapse;">
                <tr>
                    <td class="aligncenter content-block">
                        Questions? Email <a href="mailto:{{ support_email }}">{{ support_email }}</a> or visit our <a href="{{ help_center_link }}">Help Center</a>.
                    </td>
                </tr>
                 <tr>
                    <td class="aligncenter content-block" style="padding-top: 10px;">
                        {{ company_name }}<br>
                        {{ company_address_line1 }} {{ company_address_line2 }}<br>
                        © {{ current_year }} {{ company_name }}. All rights reserved.
                    </td>
                </tr>
            </table>
        </div>

    </center>
</body>
</html>
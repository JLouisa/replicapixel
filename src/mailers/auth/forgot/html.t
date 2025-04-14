<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <meta http-equiv="X-UA-Compatible" content="IE=edge"> <!-- Enables optimal rendering on some older Microsoft clients -->
    <title>Reset Your {{company}} Password</title>
    <!--[if mso]>
    <style type="text/css">
        table, td, h1, p, a {font-family: Arial, sans-serif !important;}
        /* Add any Outlook-specific overrides here */
    </style>
    <![endif]-->
    <style type="text/css">
        /* Basic Resets */
        body, table, td, a { -webkit-text-size-adjust: 100%; -ms-text-size-adjust: 100%; }
        table, td { mso-table-lspace: 0pt; mso-table-rspace: 0pt; }
        img { -ms-interpolation-mode: bicubic; border: 0; height: auto; line-height: 100%; outline: none; text-decoration: none; }
        body { height: 100% !important; margin: 0 !important; padding: 0 !important; width: 100% !important; background-color: #f4f4f4; }

        /* Fallback for clients that support <style> */
        a { color: #007bff; text-decoration: underline; }
        .button-link a { color: #ffffff !important; text-decoration: none !important; }
        .footer a { color: #666666; text-decoration: underline; }

        /* Hide preheader text */
        .preheader { display: none !important; visibility: hidden; opacity: 0; color: transparent; height: 0; width: 0; mso-hide: all; }

    </style>
</head>
<body style="margin: 0 !important; padding: 0 !important; background-color: #f4f4f4;">

    <!-- Visually Hidden Preheader Text : BEGIN -->
    <div class="preheader" style="display: none; max-height: 0; max-width: 0; opacity: 0; overflow: hidden; mso-hide: all; font-size: 1px; line-height: 1px; color: #f4f4f4;">
        Reset your password for your {{company}} account.
    </div>
    <!-- Visually Hidden Preheader Text : END -->

    <table border="0" cellpadding="0" cellspacing="0" width="100%" role="presentation">
      <!-- Outer Background Wrapper -->
      <tr>
        <td align="center" style="background-color: #f4f4f4; padding: 20px 0;">

          <!-- Main Email Content Container -->
          <table align="center" border="0" cellpadding="0" cellspacing="0" width="600" style="max-width: 600px; background-color: #ffffff; border-collapse: collapse; border: 1px solid #dddddd; margin: 0 auto;">

            <!-- Logo Section : BEGIN -->
            <tr>
              <td align="center" style="padding: 20px 0 10px 0;">
                <!-- Option 1: Image Logo (Recommended) -->
                <!-- <img src="YOUR_LOGO_URL_HERE" alt="{{company}} Logo" width="150" style="display: block; border: 0; max-width: 150px;"> -->

                <!-- Option 2: Text Logo (Fallback) -->
                <div style="color: #333333; font-family: Arial, sans-serif; font-size: 28px; font-weight: bold;">
                    {{company}}
                </div>
              </td>
            </tr>
            <!-- Logo Section : END -->

            <!-- Main Content Section : BEGIN -->
            <tr>
              <td style="padding: 20px 30px 30px 30px; color: #333333; font-family: Arial, sans-serif; font-size: 16px; line-height: 1.5;">

                <h1 style="font-size: 24px; font-weight: bold; margin: 0 0 20px 0; color: #333333; text-align: center;">Password Reset Request</h1>

                <p style="margin: 0 0 15px 0;">Dear {{name}},</p>

                <p style="margin: 0 0 15px 0;">
                  We received a request to reset the password associated with this email address. If you made this request, please click the button below to set a new password.
                </p>

                <!-- Button : BEGIN -->
                <table border="0" cellpadding="0" cellspacing="0" role="presentation" align="center" style="margin: 0 auto 25px auto;">
                  <tr>
                    <td align="center" bgcolor="#007bff" style="border-radius: 5px;" class="button-link">
                      <a href="http://{{domain}}/reset#{{resetToken}}" target="_blank" style="font-size: 16px; font-family: Arial, sans-serif; color: #ffffff; text-decoration: none; background-color: #007bff; border: 15px solid #007bff; border-radius: 5px; display: inline-block; font-weight: bold;">
                        Reset Your Password
                      </a>
                    </td>
                  </tr>
                </table>
                <!-- Button : END -->

                <p style="margin: 0 0 15px 0; font-size: 14px; text-align: center; color: #666666;">
                  If the button above doesn't work, copy and paste the following link into your web browser:
                </p>
                <p style="margin: 0 0 25px 0; font-size: 12px; text-align: center; word-break: break-all;">
                  <a href="http://{{domain}}/reset#{{resetToken}}" target="_blank" style="color: #007bff; text-decoration: underline;">
                    http://{{domain}}/reset#{{resetToken}}
                  </a>
                </p>

                 <p style="margin: 0 0 25px 0; font-weight: bold;">
                   If you did not request a password reset, please ignore this email or contact support if you have concerns. This password reset link is only valid for a limited time. <!-- Optional: Specify time limit, e.g., "for the next 2 hours" -->
                 </p>

                <p style="margin: 0;">Best regards,<br />The {{company}} Team</p>

              </td>
            </tr>
            <!-- Main Content Section : END -->

            <!-- Footer Section : BEGIN -->
            <tr>
              <td align="center" style="padding: 20px 30px; background-color: #eeeeee; color: #666666; font-family: Arial, sans-serif; font-size: 12px; line-height: 1.4;" class="footer">
                <p style="margin: 0 0 10px 0;">
                  © {{current_year}} {{company}}. All rights reserved.
                </p>
                <p style="margin: 0 0 10px 0;">
                  {{company_address}} <!-- Add your company's physical address -->
                </p>
                 <!-- Optional: Add social links if desired -->
                <!--
                <div class="social-links" style="margin-bottom: 10px;">
                  <a href="{{twitter_url}}" target="_blank" style="color: #666666; text-decoration: underline; margin: 0 5px;">Twitter</a> • 
                  <a href="{{facebook_url}}" target="_blank" style="color: #666666; text-decoration: underline; margin: 0 5px;">Facebook</a> • 
                  <a href="{{linkedin_url}}" target="_blank" style="color: #666666; text-decoration: underline; margin: 0 5px;">LinkedIn</a>
                </div>
                -->
                 <p style="margin: 0;">
                   If you're having trouble with your account, please contact <a href="mailto:{{support_email}}" style="color: #666666; text-decoration: underline;">{{support_email}}</a>. <!-- Add support email -->
                 </p>
              </td>
            </tr>
            <!-- Footer Section : END -->

          </table>
          <!-- /Main Email Content Container -->

        </td>
      </tr>
      <!-- /Outer Background Wrapper -->
    </table>

</body>
</html>
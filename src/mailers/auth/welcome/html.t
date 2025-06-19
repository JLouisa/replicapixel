<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <meta http-equiv="X-UA-Compatible" content="IE=edge"> <!-- Enables optimal rendering on some older Microsoft clients -->
    <title>Welcome to {{options.website.website_basic_info.name}}! Please Verify Your Email</title>
    <!--[if mso]>
    <style type="text/css">
        table, td, h1, p, a {font-family: Arial, sans-serif !important;}
        /* Add any Outlook-specific overrides here */
    </style>
    <![endif]-->
    <style type="text/css">
        /* Basic Resets - Applied via attributes later where needed */
        body, table, td, a { -webkit-text-size-adjust: 100%; -ms-text-size-adjust: 100%; }
        table, td { mso-table-lspace: 0pt; mso-table-rspace: 0pt; }
        img { -ms-interpolation-mode: bicubic; border: 0; height: auto; line-height: 100%; outline: none; text-decoration: none; }
        body { height: 100% !important; margin: 0 !important; padding: 0 !important; width: 100% !important; background-color: #f4f4f4; }

        /* Fallback for clients that support <style> */
        a { color: #007bff; text-decoration: underline; }
        .button-link a { color: #ffffff !important; text-decoration: none !important; } /* Ensure button text is white */
        .footer a { color: #666666; text-decoration: underline; }

        /* Hide preheader text */
        .preheader { display: none !important; visibility: hidden; opacity: 0; color: transparent; height: 0; width: 0; mso-hide: all; }

        .header {
            padding: 20px 30px;
            text-align: center;
            border-bottom: 1px solid #eaeaea;
            background-color: #c7c5c5;
        }
        .header img {
            max-width: 180px;
        }
    </style>
</head>
<body style="margin: 0 !important; padding: 0 !important; background-color: #f4f4f4;">
        <table role="presentation" class="wrapper" border="0" cellpadding="0" cellspacing="0" align="center">
            <tr>
                <td class="header">
                    <a href="{{options.website.website_basic_info.site}}" target="_blank">
                        <img src="{{ options.logo_url }}" alt="{{options.website.website_basic_info.name}} Logo" width="100">
                    </a>
                </td>
            </tr>
        </table>
    <!-- Visually Hidden Preheader Text : BEGIN -->
    <div class="preheader" style="display: none; max-height: 0; max-width: 0; opacity: 0; overflow: hidden; mso-hide: all; font-size: 1px; line-height: 1px; color: #f4f4f4;">
        Verify your email address to complete your {{options.website.website_basic_info.name}} registration!
    </div>
    <!-- Visually Hidden Preheader Text : END -->

    <table border="0" cellpadding="0" cellspacing="0" width="100%" role="presentation">
      <!-- Outer Background Wrapper -->
      <tr>
        <td align="center" style="background-color: #f4f4f4; padding: 20px 0;">

          <!-- Main Email Content Container -->
          <!-- Set a max-width for larger screens, allows shrinking on mobile -->
          <table align="center" border="0" cellpadding="0" cellspacing="0" width="600" style="max-width: 600px; background-color: #ffffff; border-collapse: collapse; border: 1px solid #dddddd; margin: 0 auto;">

            <!-- Logo Section : BEGIN -->
            <tr>
              <td align="center" style="padding: 20px 0 10px 0;">
                <!-- Option 1: Image Logo (Recommended) -->
                <!-- <img src="YOUR_LOGO_URL_HERE" alt="{{options.website.website_basic_info.name}} Logo" width="150" style="display: block; border: 0; max-width: 150px;"> -->

                <!-- Option 2: Text Logo (Fallback) -->
                <div style="color: #333333; font-family: Arial, sans-serif; font-size: 28px; font-weight: bold;">
                    {{options.website.website_basic_info.name}}
                </div>
              </td>
            </tr>
            <!-- Logo Section : END -->

            <!-- Main Content Section : BEGIN -->
            <tr>
              <td style="padding: 20px 30px 30px 30px; color: #333333; font-family: Arial, sans-serif; font-size: 16px; line-height: 1.5;">

                <h1 style="font-size: 24px; font-weight: bold; margin: 0 0 20px 0; color: #333333; text-align: center;">Welcome to {{options.website.website_basic_info.name}}!</h1>

                <p style="margin: 0 0 15px 0;">Dear {{options.user.name}},</p>

                <p style="margin: 0 0 15px 0;">
                  Thank you for joining {{options.website.website_basic_info.name}}! We're excited to have you as part of our community.
                </p>

                <p style="margin: 0 0 25px 0;">
                  To complete your registration and secure your account, please verify your email address by clicking the button below:
                </p>

                <!-- Button : BEGIN -->
                <table border="0" cellpadding="0" cellspacing="0" role="presentation" align="center" style="margin: 0 auto 25px auto;">
                  <tr>
                    <td align="center" bgcolor="#007bff" style="border-radius: 5px;" class="button-link">
                      <a href="{{options.website.website_basic_info.site | safe}}/api/auth/verify/{{options.token}}" target="_blank" style="font-size: 16px; font-family: Arial, sans-serif; color: #ffffff; text-decoration: none; background-color: #007bff; border: 15px solid #007bff; border-radius: 5px; display: inline-block; font-weight: bold;">
                        Verify Your Account
                      </a>
                    </td>
                  </tr>
                </table>
                <!-- Button : END -->

                <p style="margin: 0 0 15px 0; font-size: 14px; text-align: center; color: #666666;">
                  If the button above doesn't work, copy and paste the following link into your web browser:
                </p>
                <p style="margin: 0 0 25px 0; font-size: 12px; text-align: center; word-break: break-all;">
                  <a href="{{options.website.website_basic_info.site | safe}}/api/auth/verify/{{options.token}}" target="_blank" style="color: #007bff; text-decoration: underline;">
                    {{options.website.website_basic_info.site | safe}}/api/auth/verify/{{options.token}}
                  </a>
                </p>

                 <p style="margin: 0 0 15px 0;">
                   Once verified, you'll be able to log in and start exploring everything {{options.website.website_basic_info.name}} has to offer.
                 </p>

                <p style="margin: 0;">Best regards,<br />The {{options.website.website_basic_info.name}} Team</p>

              </td>
            </tr>
            <!-- Main Content Section : END -->

            <!-- Footer Section : BEGIN -->
            <tr>
              <td align="center" style="padding: 20px 30px; background-color: #eeeeee; color: #666666; font-family: Arial, sans-serif; font-size: 12px; line-height: 1.4;" class="footer">
                <p style="margin: 0 0 10px 0;">
                  © {{options.current_year}} {{options.website.website_basic_info.name}}. All rights reserved.
                </p>
                <p style="margin: 0 0 10px 0;">
                  {{options.company_address_line1}}
                </p>
                <div class="social-links" style="margin-bottom: 10px;">
                  <!-- Replace # with your actual social media links -->
                  <a href="{{options.twitter_url | safe}}" target="_blank" style="color: #666666; text-decoration: underline; margin: 0 5px;">Twitter</a> • 
                  <a href="{{options.facebook_url | safe}}" target="_blank" style="color: #666666; text-decoration: underline; margin: 0 5px;">Facebook</a> • 
                  <a href="{{options.linkedin_url | safe}}" target="_blank" style="color: #666666; text-decoration: underline; margin: 0 5px;">LinkedIn</a>
                </div>
                <!-- Optional: Add Unsubscribe link if appropriate for your context -->
                <!-- <p style="margin: 0;"><a href="{{options.website.website_basic_info.site | safe}}/unsubscribe_url" target="_blank" style="color: #666666; text-decoration: underline;">Unsubscribe</a></p> -->
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
<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <title>Verify Your Email | {{options.website.website_basic_info.name}}</title>
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <style>
    body {
      margin: 0;
      padding: 0;
      background-color: #f4f4f4;
      font-family: Arial, sans-serif;
    }
    table {
      border-collapse: collapse;
    }
    a {
      color: #007bff;
      text-decoration: underline;
    }
    .button-link a {
      color: #ffffff !important;
      text-decoration: none !important;
    }
  </style>
</head>
<body>

  <table border="0" cellpadding="0" cellspacing="0" width="100%">
    <tr>
      <td align="center" style="padding: 20px 0; background-color: #f4f4f4;">

        <table border="0" cellpadding="0" cellspacing="0" width="600" style="background-color: #ffffff; border: 1px solid #dddddd; padding: 30px;">
          
          <tr>
            <td align="center" style="font-size: 24px; font-weight: bold; padding-bottom: 20px;">
              {{options.website.website_basic_info.name}}
            </td>
          </tr>

          <tr>
            <td style="font-size: 16px; color: #333333; line-height: 1.5; padding: 20px 30px;">
              <p>Hello {{options.user.name}},</p>
              <p>
                You recently requested a new verification link. Please confirm your email address by clicking the button below:
              </p>

              <!-- Button -->
              <table border="0" cellpadding="0" cellspacing="0" align="center" style="margin: 20px auto;">
                <tr>
                  <td align="center" bgcolor="#007bff" style="border-radius: 5px;" class="button-link">
                    <a href="{{ options.website.website_basic_info.site | safe }}/api/auth/verify/{{options.token}}" target="_blank" style="font-size: 16px; font-weight: bold; border: 15px solid #007bff; border-radius: 5px; background-color: #007bff; display: inline-block;">
                      Verify Your Account
                    </a>
                  </td>
                </tr>
              </table>

              <p style="text-align: center; font-size: 14px; color: #666666;">
                Or copy and paste this URL into your browser:
              </p>
              <p style="text-align: center; font-size: 12px; word-break: break-all;">
                <a href="{{ options.website.website_basic_info.site | safe }}/api/auth/verify/{{options.token}}" target="_blank">
                  {{ options.website.website_basic_info.site | safe }}/api/auth/verify/{{options.token}}
                </a>
              </p>

              <p>
                If you did not request this email, you can safely ignore it.
              </p>

              <p>Best regards,<br />The {{options.website.website_basic_info.name}} Team</p>
            </td>
          </tr>

        </table>

      </td>
    </tr>
  </table>

</body>
</html>

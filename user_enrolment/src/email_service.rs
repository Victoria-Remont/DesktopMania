use lettre::{
    Email, 
    SmtpClient,
    ClientSecurity,
    ClientTlsParameters,
    Transport,
    smtp::{
        ConnectionReuseParameters,
        authentication::{Credentials,Mechanism}
    }
};

use native_tls::{Protocol, TlsConnector};
use crate::{
    model::Confirmation,errors::AuthError,config
};


pub fn send_confirmation_mail(confirmation:&Confirmation) -> Result<(),AuthError> {
    let domain_url = config::domain_url();
    let expires = confirmation.expires_at.format("%I:%M%p %A %-d %B %C%y").to_string();
    let html_text = format!("Please click on the link below to complete registration. <br/>
    <a href=\"{domain}/register?id={id}&email={email}\">Complete registration</a><br/>This link expires on <strong>{expires}</strong>",
    domain=domain_url,
    id=confirmation.id,
    email=confirmation.email,
    expires=expires
    );
    
    let plain_text = format!(
        "Please visit the link below to complete registration:\n
        {domain}/register.html?id={id}&email={email}\n
        This link expires on {expires}.",
        domain=domain_url,
        id=confirmation.id,
        email=confirmation.email,
        expires=expires
    );

    let email = Email::builder()
    .to(confirmation.email.clone())
    .from(("noreply@auth-service.com",config::smtp_sender_name()))
    .subject("Complete your registration")
    .text(plain_text)
    .html(html_text)
    .build()
    .unwrap();

    let smtp_host = config::smtp_host();
    let mut tls_builder = TlsConnector::builder();

    //TODO update protocol 
    tls_builder.min_protocol_version(Some(Protocol::Tlsv10));

    let tls_parameters = ClientTlsParameters::new(smtp_host.clone(),
    tls_builder.build().unwrap());

    let mut mailer = SmtpClient::new((smtp_host.as_str(), config::smtp_port()),
    ClientSecurity::Required(tls_parameters)).unwrap()
    .authentication_mechanism(Mechanism::Login)
    .Credentials(Credentials::new(config::smtp_username(), config::smtp_password()))

    .connection_reuse(ConnectionReuseParameters::ReuseUnlimited)
    .transport();
    let result = mailer.send(email);

    if result.is_ok(){
        println!("Email sent");
        ok(())
    }
    else{
        println!("Could not send mail: {:?}",result);

        Err(AuthError::ProcessError(String::from("Could not send confirmation mail")))
    }

}
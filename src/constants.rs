/// Sample Xmpp pubsub messages for testing purpose

/// Sample Xmpp publish message without published items
pub const XMPP_PUBSUB_PUBLISH_EMPTY: &str =
    r#"<pubsub xmlns='http://jabber.org/protocol/pubsub'><publish node='hello'/></pubsub>"#;

/// Sample Xmpp pubsub message containing a published item and publish options
pub const XMPP_PUBSUB_PUBLISH_WITH_OPTIONS: &str = r#"<pubsub xmlns='http://jabber.org/protocol/pubsub'>
    <publish node="jedi">
      <item>
        <notification xmlns='urn:xmpp:push:0'>
          <x xmlns='jabber:x:data' type="form">
            <field type="hidden" var="FORM_TYPE">
              <value>urn:xmpp:push:summary</value>
            </field>
            <field type="text-single" var="message-count">
              <value>1</value>
            </field>
            <field type="text-single" var="pending-subscription-count"/>
            <field type="jid-single" var="last-message-sender">
              <value>toto@foo.org</value>
            </field>
            <field type="text-single" var="last-message-body">
              <value>My first message</value>
            </field>
            <field type="jid-single" var="last-message-recipient">
              <value>tata@foo.org</value>
            </field>
            <field type='text-multi' var='description'>
              <value>First description line</value>
              <value>Second description line</value>
            </field>
          </x>
        </notification>
      </item>
    </publish>
    <publish-options>
      <x xmlns='jabber:x:data' type="submit">
        <field type="hidden" var="FORM_TYPE">
          <value>http://jabber.org/protocol/pubsub#publish-options</value>
        </field>
      </x>
    </publish-options>
  </pubsub>
"#;

/// Sample Xmpp publish message containing an item but no options
pub const XMPP_PUBSUB_PUBLISH_WITHOUT_OPTIONS: &str = r#"<pubsub xmlns='http://jabber.org/protocol/pubsub'>
<publish node="jedi">
  <item>
    <notification xmlns='urn:xmpp:push:0'>
      <x xmlns='jabber:x:data' type="form">
        <field type="hidden" var="FORM_TYPE">
          <value>urn:xmpp:push:summary</value>
        </field>
        <field type="text-single" var="message-count">
          <value>1</value>
        </field>
        <field type="text-single" var="pending-subscription-count"/>
        <field type="jid-single" var="last-message-sender">
          <value>toto@foo.org</value>
        </field>
        <field type="text-single" var="last-message-body">
          <value>Mon super message</value>
        </field>
        <field type="jid-single" var="last-message-recipient">
          <value>tata@foo.org</value>
        </field>
      </x>
    </notification>
  </item>
</publish>
</pubsub>
"#;

/// Field element values
pub const LAST_MESSAGE_SENDER: &str = r#"last-message-sender"#;
pub const LAST_MESSAGE_RECIPIENT: &str = r#"last-message-recipient"#;
pub const LAST_MESSAGE_BODY: &str = r#"last-message-body"#;

pub const DESCRIPTION: &str = r#"description"#;

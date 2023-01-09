# Spring JMS

How to ensure the destination is of the right type? (Queue or topic)

- Set pubSubDomain on JmsTemplate?
- Custom DestinationResolver?

@JmsListener

- subscription = “blah” - durable subscriber ID

While `SimpleMessageListenerContainer`does not allow for the participation in externally managed transactions, it does support native JMS transactions: simply switch the 'sessionTransacted' flag to 'true' or, in the namespace, set the 'acknowledge' attribute to 'transacted': Exceptions thrown from your listener will lead to a rollback then, with the message getting redelivered. Alternatively, consider using 'CLIENT_ACKNOWLEDGE' mode which provides redelivery in case of an exception as well but does not use transacted Sessions and therefore does not include any other Session operations (such as sending response messages) in the transaction protocol.

**The default 'AUTO_ACKNOWLEDGE' mode does not provide proper reliability guarantees.** Messages may get lost when listener execution fails (since the provider will automatically acknowledge each message after listener invocation, with no exceptions to be propagated to the provider) or when the listener container shuts down (this may be configured through the 'acceptMessagesWhileStopping' flag). Make sure to use transacted sessions in case of reliability needs, e.g. for reliable queue handling and durable topic subscriptions

Like its sibling `SimpleMessageListenerContainer`, `DefaultMessageListenerContainer`supports native JMS transactions and also allows for customizing the acknowledgment mode. This is strongly recommended over externally managed transactions if feasible for your scenario: that is, if you can live with occasional duplicate messages in case of the JVM dying. Custom duplicate message detection steps in your business logic may cover such situations, e.g. in the form of a business entity existence check or a protocol table check. Any such arrangements will be significantly more efficient than the alternative: wrapping your entire processing with an XA transaction (through configuring your `DefaultMessageListenerContainer`with an `JtaTransactionManager`), covering the reception of the JMS message as well as the execution of the business logic in your message listener (including database operations etc).**The default 'AUTO_ACKNOWLEDGE' mode does not provide proper reliability guarantees.** Messages may get lost when listener execution fails (since the provider will automatically acknowledge each message before listener invocation) or when the listener container shuts down (this may be configured through the 'acceptMessagesWhileStopping' flag). Make sure to use transacted sessions in case of reliability needs, e.g. for reliable queue handling and durable topic subscriptions.
use tracing_subscriber::Layer;

pub struct MastsOtelLayer;

#[derive(Debug)]
struct MastsOtelStorage(Vec<opentelemetry::KeyValue>);

impl<S> tracing_opentelemetry::OpenTelemetryLayer<S> for MastsOtelLayer where S: tracing::Subscriber {}

impl<S> tracing_opentelemetry::OpenTelemetryLayer<S> for MastsOtelLayer
where
    S: tracing::Subscriber,
    S: for<'lookup> tracing_subscriber::registry::LookupSpan<'lookup>,
{
    //Records OpenTelemetry [`Event`] data on event.
    //
    // Note:
    // An ERROR-level event will also set the OpenTelemetry span
    // status code to Error, signaling that an error has occurred.
    //
    fn on_event(&self, event: &tracing::Event<'_>, ctx: tracing_subscriber::layer::Context<'_, S>) {
        // All of the span context
        let scope = ctx.event_scope(event).unwrap();
        let mut spans = vec![];
        for span in scope.from_root() {
            let extensions = span.extensions();
            let storage = extensions.get::<MastsOtelStorage>().unwrap();
            let field_data: &Vec<opentelemetry::KeyValue> = &storage.0;
            spans.push(serde_json::json!({
                "target": span.metadata().target(),
                "name": span.name(),
                "level": format!("{:?}", span.metadata().level()),
                "fields": field_data,
            }));
        }

        // The fields of the event
        let mut fields = vec![opentelemetry::KeyValue];
        let mut visitor = MastsOtelVisitor(&mut fields);
        event.record(&mut visitor);

        // And create our output
        let output = serde_json::json!({
            "target": event.metadata().target(),
            "name": event.metadata().name(),
            "level": format!("{:?}", event.metadata().level()),
            "fields": fields,
            "spans": spans,
        });
        println!("{}", serde_json::to_string_pretty(&output).unwrap());
    }

    // Creates an OpenTelemetry [`Span`] for the corresponding tracing Span.
    fn on_new_span(
        &self,
        attrs: &tracing::span::Attributes<'_>,
        id: &tracing::span::Id,
        ctx: tracing_subscriber::layer::Context<'_, S>,
    ) {
        let span = ctx.span(id).unwrap();
        println!("Got on_new_span!");
        println!("  level={:?}", span.metadata().level());
        println!("  target={:?}", span.metadata().target());
        println!("  name={:?}", span.metadata().name());

        // Our old friend, `println!` exploration.
        let mut visitor = MastsOtelVisitor;
        attrs.record(&mut visitor);
        // Build our json object from the field values like we have been
        let mut fields = BTreeMap::new();
        let mut visitor = MastsOtelVisitor(&mut fields);
        attrs.record(&mut visitor);

        // And stuff it in our newtype.
        let storage = MastsOtelStorage(fields);

        // Get a reference to the internal span data
        let span = ctx.span(id).unwrap();
        // Get the special place where tracing stores custom data
        let mut extensions = span.extensions_mut();
        // And store our data
        extensions.insert::<MastsOtelStorage>(storage);
    }

    // Record OpenTelemetry [`attributes`] for the given values.
    fn on_record(
        &self,
        id: &tracing::span::Id,
        values: &tracing::span::Record<'_>,
        ctx: tracing_subscriber::layer::Context<'_, S>,
    ) {
        // Get the span whose data is being recorded
        let span = ctx.span(id).unwrap();

        // Get a mutable reference to the data we created in on_new_span
        let mut extensions_mut = span.extensions_mut();
        let custom_field_storage: &mut MastsOtelStorage =
            extensions_mut.get_mut::<MastsOtelStorage>().unwrap();
        let json_data: &mut Vec<String, serde_json::Value> = &mut custom_field_storage.0;

        // And add to using our old friend the visitor!
        let mut visitor = MastsOtelVisitor(json_data);
        values.record(&mut visitor);
    }
}

// Visitor Pattern: Implement `Visit` to get the values from events.
// [`Visit`] exposes a `record_*` method for each type
// tracing can handle.
struct MastsOtelVisitor<'a>(&'a mut BTreeMap<String, serde_json::Value>);

impl tracing::field::Visit for MastsOtelVisitor {
    fn record_error(
        &mut self,
        field: &tracing::field::Field,
        value: &(dyn std::error::Error + 'static),
    ) {
        println!("  field={} value={}", field.name(), value)
    }

    fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
        println!("  field={} value={:?}", field.name(), value)
    }

    fn on_event(
        &self,
        event: &tracing::Event<'_>,
        _ctx: tracing_subscriber::layer::Context<'_, S>,
    ) {
        println!("Got event!");
        println!("  level={:?}", event.metadata().level());
        println!("  target={:?}", event.metadata().target());
        println!("  name={:?}", event.metadata().name());
        let mut visitor = PrintlnVisitor;
        event.record(&mut visitor);
    }
}

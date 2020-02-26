use opcua_types::*;

use crate::{
    address_space::address_space::AddressSpace,
    events::event::{BaseEventType, Event},
};

use super::AuditEvent;

/// Base type for audit events. Do not raise events of this type
pub(super) struct AuditEventType {
    base: BaseEventType,
    action_time_stamp: UtcTime,
    status: bool,
    server_id: UAString,
    client_audit_entry_id: UAString,
    client_user_id: UAString,
    // This vector is used to capture properties of the event as they are raised in order. This can
    // be used by the audit event to produce a log message.
    properties: Vec<(String, Variant)>,
}

impl AuditEvent for AuditEventType {
    fn event_type_id() -> NodeId {
        panic!();
    }

    fn log_message(&self) -> String {
        format!("")
    }
}

impl Event for AuditEventType {
    type Err = ();

    fn is_valid(&self) -> bool {
        self.base.is_valid()
    }

    fn raise(self, address_space: &mut AddressSpace) -> Result<NodeId, Self::Err>
    {
        if self.is_valid() {
            let node_id = self.base.raise(address_space)?;
            let ns = node_id.namespace;
            Self::add_property(&node_id, NodeId::next_numeric(ns), "ActionTimeStamp", "ActionTimeStamp", self.action_time_stamp.clone(), address_space);
            Self::add_property(&node_id, NodeId::next_numeric(ns), "Status", "Status", self.status, address_space);
            Self::add_property(&node_id, NodeId::next_numeric(ns), "ServerId", "ServerId", self.server_id.clone(), address_space);
            Self::add_property(&node_id, NodeId::next_numeric(ns), "ClientAuditEntryId", "ClientAuditEntryId", self.client_audit_entry_id.clone(), address_space);
            Self::add_property(&node_id, NodeId::next_numeric(ns), "ClientUserId", "ClientUserId", self.client_user_id.clone(), address_space);
            Ok(node_id)
        } else {
            error!("AuditEventType is invalid and will not be inserted");
            Err(())
        }
    }

    fn add_property<T, R, S, V>(event_id: &NodeId, property_id: T, browse_name: R, display_name: S, value: V, address_space: &mut AddressSpace)
        where T: Into<NodeId>,
              R: Into<QualifiedName>,
              S: Into<LocalizedText>,
              V: Into<Variant>
    {
        Event::add_property(event_id, property_id, browse_name, display_name, value, address_space);
    }
}

base_event_impl!(AuditEventType, base);

impl AuditEventType {
    pub fn new<R, E, S, T>(node_id: R, event_type_id: E, browse_name: S, display_name: T, time: DateTime) -> Self
        where R: Into<NodeId>,
              E: Into<NodeId>,
              S: Into<QualifiedName>,
              T: Into<LocalizedText>,
    {
        let action_time_stamp = DateTime::now();
        let server_id = UAString::null();
        let parent_node = Self::parent_node();
        Self {
            base: BaseEventType::new(node_id, event_type_id, browse_name, display_name, parent_node, time),
            status: false,
            action_time_stamp,
            server_id,
            client_audit_entry_id: UAString::null(),
            client_user_id: UAString::null(),
            properties: Vec::with_capacity(10)
        }
    }

    pub fn client_audit_entry_id<T>(mut self, client_audit_entry_id: T) -> Self where T: Into<UAString> {
        self.client_audit_entry_id = client_audit_entry_id.into();
        self
    }

    pub fn client_user_id<T>(mut self, client_user_id: T) -> Self where T: Into<UAString> {
        self.client_user_id = client_user_id.into();
        self
    }

    pub fn status(mut self, status: bool) -> Self {
        self.status = status;
        self
    }

    pub fn server_id<T>(mut self, server_id: T) -> Self where T: Into<UAString> {
        self.server_id = server_id.into();
        self
    }

    pub fn action_time_stamp(mut self, action_time_stamp: UtcTime) -> Self {
        self.action_time_stamp = action_time_stamp;
        self
    }
}

macro_rules! audit_event_impl {
    ( $event:ident, $base:ident ) => {
        base_event_impl!($event, $base);

        impl $event {
            pub fn client_audit_entry_id<T>(mut self, client_audit_entry_id: T) -> Self where T: Into<UAString> {
                self.$base = self.$base.client_audit_entry_id(client_audit_entry_id);
                self
            }

            pub fn client_user_id<T>(mut self, client_user_id: T) -> Self where T: Into<UAString> {
                self.$base = self.$base.client_user_id(client_user_id);
                self
            }

            pub fn status(mut self, status: bool) -> Self {
                self.$base = self.$base.status(status);
                self
            }

            pub fn server_id<T>(mut self, server_id: T) -> Self where T: Into<UAString> {
                self.$base = self.$base.server_id(server_id);
                self
            }

            pub fn action_time_stamp(mut self, action_time_stamp: UtcTime) -> Self {
                self.$base = self.$base.action_time_stamp(action_time_stamp);
                self
            }
        }
    }
}
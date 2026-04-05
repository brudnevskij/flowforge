CREATE TABLE orders (
    id UUID PRIMARY KEY,

    customer_reference TEXT NOT NULL,
    partner_name TEXT NOT NULL,
    external_reference TEXT,

    status TEXT NOT NULL,

    amount_cents BIGINT NOT NULL CHECK (amount_cents >= 0),
    currency TEXT NOT NULL,

    failure_reason TEXT,

    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),

    submitted_at TIMESTAMPTZ,
    completed_at TIMESTAMPTZ,
    cancelled_at TIMESTAMPTZ
);

CREATE TABLE audit_logs (
    id UUID PRIMARY KEY,

    order_id UUID NOT NULL REFERENCES orders(id) ON DELETE CASCADE,

    event_type TEXT NOT NULL,
    message TEXT NOT NULL,
    metadata JSONB,

    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- indexes

CREATE INDEX idx_orders_status ON orders(status);

CREATE INDEX idx_orders_created_at ON orders(created_at);

CREATE INDEX idx_audit_logs_order_id ON audit_logs(order_id);


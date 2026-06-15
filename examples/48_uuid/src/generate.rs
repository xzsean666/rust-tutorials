//! 生成 UUID：随机 v4 UUID，以及给业务实体分配新鲜 ID。

use uuid::Uuid;

/// 生成一个全新的 v4（随机）UUID。
///
/// v4 UUID 的绝大部分位来自随机数，因此每次调用几乎一定得到不同的值，
/// 不需要中心化的协调就能保证唯一。
///
/// ```
/// let a = rt_48_uuid::new_id();
/// let b = rt_48_uuid::new_id();
/// assert_ne!(a, b); // 两次生成几乎一定不同
/// ```
pub fn new_id() -> Uuid {
    Uuid::new_v4()
}

/// 一次性生成 `n` 个互不相同的 v4 UUID。
pub fn new_ids(n: usize) -> Vec<Uuid> {
    (0..n).map(|_| new_id()).collect()
}

/// 一个业务实体：拥有一个唯一 `id` 和一个 `name`。
///
/// 典型用法是“新建对象时自动分配 ID”，调用方只需提供名字。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entity {
    /// 实体的唯一标识，由 [`Entity::new`] 自动生成。
    pub id: Uuid,
    /// 实体名字。
    pub name: String,
}

impl Entity {
    /// 创建一个新实体，并为它分配一个全新的 v4 UUID。
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            id: new_id(),
            name: name.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Version;

    #[test]
    fn new_id_is_v4() {
        let id = new_id();
        // v4 表示“随机”版本，对应版本号 4。
        assert_eq!(id.get_version(), Some(Version::Random));
        assert_eq!(id.get_version_num(), 4);
    }

    #[test]
    fn two_ids_differ() {
        assert_ne!(new_id(), new_id());
    }

    #[test]
    fn new_ids_returns_distinct_values() {
        let ids = new_ids(5);
        assert_eq!(ids.len(), 5);
        // 收进 HashSet 后数量不变，说明没有重复。
        let unique: std::collections::HashSet<_> = ids.iter().collect();
        assert_eq!(unique.len(), 5);
    }

    #[test]
    fn entity_gets_fresh_id() {
        let a = Entity::new("alice");
        let b = Entity::new("bob");
        assert_eq!(a.name, "alice");
        assert_ne!(a.id, b.id); // 每个实体的 ID 都不同
    }
}

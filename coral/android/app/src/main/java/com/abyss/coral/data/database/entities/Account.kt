package com.abyss.coral.data.database.entities

import android.os.Parcelable
import androidx.room.ColumnInfo
import androidx.room.Entity
import androidx.room.Index
import androidx.room.PrimaryKey
import kotlinx.parcelize.Parcelize

@Entity(indices=[(Index(value=arrayOf("brokerID", "userID"), unique=true))])
@Parcelize
data class Account(
    @ColumnInfo(name = "id")
    @PrimaryKey(autoGenerate=true)
    var id: Long = 0,

    @ColumnInfo(name = "brokerID")
    var brokerID: String,

    @ColumnInfo(name = "userID")
    var userID: String,

    @ColumnInfo(name = "password")
    var password: String,
): Parcelable
